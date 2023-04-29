use yaml_rust::{yaml, Yaml, YamlLoader};

use crate::{
    core::{
        antialias::{AAMethod, AntiAliasing, Multisampling, Stochastic},
        camera::Camera,
        light::Light,
        material::Material,
        pattern::Pattern,
        world::World,
    },
    math::{Axis, Matrix, Point, Vec3},
    shape::{Plane, Shape, Sphere},
    visuals::Color,
};

use super::error::ParseResult;

/// Attempts to parse the specified YAML file. Scans the file for items of the form `- add: item`.
/// Can fail when reading the file to string or when scanning the file with
/// [YamlLoader](yaml_rust::YamlLoader::load_from_str).
///
/// # Example
/// ```ignore
/// let (camera, world) = parse_yaml("world.yml").unwrap();
///
/// let canvas = camera.unwrap().render(&world).unwrap();
/// canvas.export("rendered_image.png").unwrap();
/// ```
pub fn parse_yaml<P>(path: P) -> ParseResult<Camera, World>
where
    P: AsRef<std::path::Path>,
{
    let yaml = std::fs::read_to_string(path)?;
    let docs = YamlLoader::load_from_str(&yaml)?;
    let doc = &docs[0];

    let mut camera = None;
    let mut shapes: Vec<Shape> = Vec::new();
    let mut lights: Vec<Light> = Vec::new();

    for elem in doc.as_vec().unwrap().iter() {
        let hash = elem.as_hash().unwrap();

        // look for "- add: item" in the yaml file
        if let Some(item) = hash.get(&Yaml::from_str("add")) {
            let t = item.as_str().unwrap();

            match t {
                "camera" => {
                    camera = make_camera(hash);
                }
                "light" => {
                    lights.push(make_light(hash).expect("could not parse lights"));
                }
                "sphere" | "plane" => {
                    shapes.push(make_shape(hash, t).expect("could not parse shapes"));
                }
                _ => unimplemented!("item type {:?} was not recognized", t),
            }
        }
    }

    let world = World::new(shapes, lights);

    Ok((camera, world))
}

/// Constructs a camera from the data in the current hash. Returns `None` if any of `hsize`,
/// `vsize`, `fov`, `from`, `up`, or `to` are missing. TODO: This probably isn't desired, so there
/// should be defaults in the future.
fn make_camera(hash: &yaml::Hash) -> Option<Camera> {
    let hsize = usize_from_key(hash, "hsize")?;
    let vsize = usize_from_key(hash, "vsize")?;
    let fov = float_from_key(hash, "fov")?;

    let from = point_from_key(hash, "from")?;
    let to = point_from_key(hash, "to")?;
    let up = vec3_from_key(hash, "up")?;
    let aa = set_antialiasing(hash)?;

    Some(
        Camera::new(hsize, vsize, fov)
            .with_antialiasing(aa.level)
            .with_aa_method(aa.method)
            .with_transform(&Matrix::view_transform(from, to, up)),
    )
}

fn set_antialiasing(hash: &yaml::Hash) -> Option<AntiAliasing> {
    let default = AntiAliasing::default();

    if let Some(aa) = hash.get(&Yaml::from_str("aa")) {
        let aa_hash = aa
            .as_hash()
            .expect("could not parse `aa` properly in the YAML file");
        let level = usize_from_key(aa_hash, "level").unwrap_or(default.level);
        let etol = float_from_key(aa_hash, "tolerance").unwrap_or(default.error_tolerance);
        let method = aa_hash.get(&Yaml::from_str("method"))?.as_str()?;

        match method {
            "random" | "stochastic" => Some(
                default
                    .with_method(AAMethod::Stochastic(Stochastic::default()))
                    .with_level(level),
            ),

            "multisampling" | "msaa" => Some(
                default
                    .with_method(AAMethod::Multisampling(Multisampling::default()))
                    .with_tolerance(etol)
                    .with_level(level),
            ),

            _ => None,
        }
    } else {
        None
    }
}

/// Constructs a shape from a hash and a "type" keyword. Returns `None` if the "type" isn't a
/// recognized shape. TODO: refactor how `Shape` works with individual shape variants. Code right
/// now is repetitive.
fn make_shape(hash: &yaml::Hash, t: &str) -> Option<Shape> {
    match t {
        "sphere" => Some(
            Sphere::default()
                .with_material(&make_material(hash))
                .with_transform(&transform(hash))
                .as_shape(),
        ),
        "plane" => Some(
            Plane::default()
                .with_material(&make_material(hash))
                .with_transform(&transform(hash))
                .as_shape(),
        ),
        _ => None,
    }
}

/// Constructs a light from a hash. Returns `None` if the light type isn't recognized. There's only
/// one light type as of now, but this makes it easier to add more in the future.
fn make_light(hash: &yaml::Hash) -> Option<Light> {
    let t = hash.get(&Yaml::from_str("type"))?.as_str()?;

    match t {
        "point" => Some(Light::new_point_light(
            point_from_key(hash, "at")?,
            color_from_key(hash, "intensity")?,
        )),
        _ => None,
    }
}

/// Constructs a new material from a hash.
fn make_material(hash: &yaml::Hash) -> Material {
    let default = Material::default();

    if let Some(mat) = hash.get(&Yaml::from_str("material")) {
        let mat_hash = mat.as_hash().unwrap();

        Material::default()
            .with_color(&color_from_key(mat_hash, "color").unwrap_or(default.color))
            .with_pattern(&make_pattern(mat_hash, "pattern").expect("could not parse the pattern"))
            .with_ambient(float_from_key(mat_hash, "ambient").unwrap_or(default.ambient))
            .with_diffuse(float_from_key(mat_hash, "diffuse").unwrap_or(default.diffuse))
            .with_specular(float_from_key(mat_hash, "specular").unwrap_or(default.specular))
            .with_shininess(float_from_key(mat_hash, "shininess").unwrap_or(default.shininess))
            .with_reflective(float_from_key(mat_hash, "reflective").unwrap_or(default.reflective))
    } else {
        default
    }
}

/// Parse a specified transformation. If no transform is specified, uses identity matrix. Probably
/// the easiest idea is to have the data entered as:
///
///  - add: sphere
///    transform:
///      - [scale, x, y, z]
///      - [rotate-z, 1.2731]
///      - [translate, -0.25, 0.5, -0.25]
fn transform(hash: &yaml::Hash) -> Matrix<4> {
    if let Some(tf_list) = hash.get(&Yaml::from_str("transform")) {
        let tf_array = tf_list.as_vec().unwrap();
        let mut total_transformation = Matrix::identity();

        // transformations are applied in "reverse" order, but I don't think I want to put that in
        // here?
        for tf in tf_array.iter() {
            let t = tf[0].as_str().unwrap();
            match t {
                "scale" => {
                    let tm = Matrix::scaling(
                        tf[1].as_f64().unwrap(),
                        tf[2].as_f64().unwrap(),
                        tf[3].as_f64().unwrap(),
                    );
                    total_transformation = total_transformation * tm
                }
                "rotate-x" => {
                    let tm = Matrix::rotation(Axis::X, tf[1].as_f64().unwrap());
                    total_transformation = total_transformation * tm
                }
                "rotate-y" => {
                    let tm = Matrix::rotation(Axis::Y, tf[1].as_f64().unwrap());
                    total_transformation = total_transformation * tm
                }
                "rotate-z" => {
                    let tm = Matrix::rotation(Axis::Z, tf[1].as_f64().unwrap());
                    total_transformation = total_transformation * tm
                }
                "translate" => {
                    let tm = Matrix::translation(
                        tf[1].as_f64().unwrap(),
                        tf[2].as_f64().unwrap(),
                        tf[3].as_f64().unwrap(),
                    );
                    total_transformation = total_transformation * tm
                }
                "shear" => {
                    let tm = Matrix::shear(
                        tf[1].as_f64().unwrap(),
                        tf[2].as_f64().unwrap(),
                        tf[3].as_f64().unwrap(),
                        tf[4].as_f64().unwrap(),
                        tf[5].as_f64().unwrap(),
                        tf[6].as_f64().unwrap(),
                    );
                    total_transformation = total_transformation * tm
                }
                _ => {
                    eprintln!(
                        "unknown transformation specified: {:?}. Using identity matrix instead.",
                        t
                    );
                    let tm = Matrix::identity();
                    total_transformation = total_transformation * tm
                }
            }
        }

        total_transformation
    } else {
        Matrix::identity()
    }
}

/// Constructs a pattern from a hash and a keyword. The keyword argument is only to make blended
/// patterns easier to implement. YAML should look like:
///
/// - add: sphere
///   material:
///     pattern:
///       type: stripes
///       colors:
///         - [1.0, 1.0, 1.0]
///         - [0.0, 0.0, 0.0]
fn make_pattern(hash: &yaml::Hash, kw: &str) -> Option<Pattern> {
    if let Some(pat) = hash.get(&Yaml::from_str(kw)) {
        let pat_hash = pat.as_hash()?;
        let t = pat_hash.get(&Yaml::from_str("type"))?.as_str()?;

        let pat = match t {
            "stripes" | "striped" => {
                let stripe_colors = pat_hash
                    .get(&Yaml::from_str("colors"))?
                    .as_vec()?
                    .iter()
                    .map(|c| make_color(c).unwrap())
                    .collect::<Vec<_>>();

                Some(Pattern::new_stripes(stripe_colors).with_transform(&transform(pat_hash)))
            }
            "gradient" => {
                let grad_colors = pat_hash
                    .get(&Yaml::from_str("colors"))?
                    .as_vec()?
                    .iter()
                    .map(|c| make_color(c).unwrap())
                    .collect::<Vec<_>>();

                Some(
                    Pattern::new_gradient(grad_colors[0], grad_colors[1])
                        .with_transform(&transform(pat_hash)),
                )
            }
            "ring" | "rings" => {
                let ring_colors = pat_hash
                    .get(&Yaml::from_str("colors"))?
                    .as_vec()?
                    .iter()
                    .map(|c| make_color(c).unwrap())
                    .collect::<Vec<_>>();

                Some(Pattern::new_rings(ring_colors).with_transform(&transform(pat_hash)))
            }
            "checkers" | "checkered" => {
                let checker_colors = pat_hash
                    .get(&Yaml::from_str("colors"))?
                    .as_vec()?
                    .iter()
                    .map(|c| make_color(c).unwrap())
                    .collect::<Vec<_>>();

                Some(Pattern::new_checkers(checker_colors[0], checker_colors[1]))
            }
            "blend" | "blended" => {
                let bh1 = pat_hash.get(&Yaml::from_str("pattern1"))?.as_hash()?;
                let bh2 = pat_hash.get(&Yaml::from_str("pattern2"))?.as_hash()?;

                let p1 = make_pattern(pat_hash, "pattern1")?.with_transform(&transform(bh1));
                let p2 = make_pattern(pat_hash, "pattern2")?.with_transform(&transform(bh2));

                Some(Pattern::new_blended(p1, p2))
            }
            _ => None,
        };

        pat
    } else {
        None
    }
}

fn make_color(seq: &Yaml) -> Option<Color> {
    let comps = seq.as_vec()?;

    assert!(comps.len() == 3);

    Some(Color(
        comps[0].as_f64()?,
        comps[1].as_f64()?,
        comps[2].as_f64()?,
    ))
}

fn color_from_key(hash: &yaml::Hash, key: &str) -> Option<Color> {
    let seq = hash.get(&Yaml::from_str(key))?;

    make_color(seq)
}

fn vec3_from_key(hash: &yaml::Hash, key: &str) -> Option<Vec3> {
    let seq = hash.get(&Yaml::from_str(key))?;

    let comps = seq.as_vec()?;

    assert!(comps.len() == 3);

    Some(Vec3(
        comps[0].as_f64()?,
        comps[1].as_f64()?,
        comps[2].as_f64()?,
    ))
}

fn point_from_key(hash: &yaml::Hash, key: &str) -> Option<Point> {
    let seq = hash.get(&Yaml::from_str(key))?;

    let comps = seq.as_vec()?;

    assert!(comps.len() == 3);

    Some(Point(
        comps[0].as_f64()?,
        comps[1].as_f64()?,
        comps[2].as_f64()?,
    ))
}

fn float_from_key(hash: &yaml::Hash, key: &str) -> Option<f64> {
    let f = hash.get(&Yaml::from_str(key))?;

    f.as_f64()
}

fn usize_from_key(hash: &yaml::Hash, key: &str) -> Option<usize> {
    let u = hash.get(&Yaml::from_str(key))?;

    Some(u.as_i64()? as usize)
}

#[cfg(test)]
mod yaml_tests {
    use super::*;
    use crate::io::error::{ParseResult, YamlError};

    type YamlResult<T> = Result<T, YamlError>;

    fn parse_from_str(s: &str) -> ParseResult<Camera, World> {
        let docs = YamlLoader::load_from_str(s)?;
        let doc = &docs[0];

        let mut camera = None;
        let mut shapes: Vec<Shape> = Vec::new();
        let mut lights: Vec<Light> = Vec::new();

        for elem in doc.as_vec().unwrap().iter() {
            let hash = elem.as_hash().unwrap();

            // look for "- add: item" in the yaml file
            if let Some(item) = hash.get(&Yaml::from_str("add")) {
                let t = item.as_str().unwrap();

                match t {
                    "camera" => {
                        camera = make_camera(hash);
                    }
                    "light" => {
                        lights.push(make_light(hash).expect("could not parse lights"));
                    }
                    "sphere" | "plane" => {
                        shapes.push(make_shape(hash, t).expect("could not parse shapes"));
                    }
                    _ => unimplemented!("item type {:?} was not recognized", t),
                }
            }
        }

        let world = World::new(shapes, lights);

        Ok((camera, world))
    }

    #[test]
    fn can_parse_ring_patterns() -> YamlResult<()> {
        let yaml = r#"
---
- add: plane
  material:
    pattern:
      type: rings
      colors:
        - [1.0, 0.0, 0.0]
        - [0.0, 0.0, 1.0]
"#;
        let y = parse_from_str(yaml)?;
        let yw = y.1;

        assert_eq!(
            yw.objects[0].material().pattern.unwrap(),
            Pattern::new_rings(vec![Color::red(), Color::blue()])
        );

        Ok(())
    }

    #[test]
    fn can_parse_lights_from_yaml() -> YamlResult<()> {
        let yaml = r#"
---
- add: light
  type: point
  intensity: [1.0, 1.0, 1.0]
  at: [-5.0, 10.0, 0.0]
"#;
        let yl = parse_from_str(yaml)?;

        assert_eq!(
            yl.1.lights[0],
            Light::new_point_light(Point(-5.0, 10.0, 0.0), Color::white())
        );

        Ok(())
    }

    #[test]
    fn can_parse_spheres_from_yaml() -> YamlResult<()> {
        let yaml = r#"
---
- add: sphere
  material:
   pattern:
     type: stripes
     colors:
       - [1.0, 1.0, 1.0]
       - [0.0, 0.0, 0.0]
   ambient: 0.5
   diffuse: 0.25
   shininess: 0.08
"#;
        let ys = parse_from_str(yaml)?;

        assert_eq!(ys.1.objects[0].material().ambient, 0.5);
        assert_eq!(
            ys.1.objects[0].material().pattern,
            Some(Pattern::new_stripes(vec![Color::white(), Color::black()]))
        );

        Ok(())
    }

    #[test]
    fn can_make_materials_from_yaml() -> YamlResult<()> {
        let yaml = r#"
---
material:
  pattern:
    type: stripes
    colors:
      - [1.0, 0.0, 0.0]
      - [0.0, 0.0, 0.0]
  ambient: 0.5
  diffuse: 0.25
  shininess: 0.08
"#;
        let docs = YamlLoader::load_from_str(yaml)?;
        let doc = &docs[0];

        let hash = doc.as_hash().unwrap();
        let mat = make_material(hash);

        assert_eq!(
            mat.pattern,
            Some(Pattern::new_stripes(vec![Color::red(), Color::black()]))
        );
        assert_eq!(mat.ambient, 0.5);
        assert_eq!(mat.diffuse, 0.25);
        assert_eq!(mat.specular, 0.9); // the default material specular
        assert_eq!(mat.shininess, 0.08);

        Ok(())
    }

    #[test]
    fn can_make_points_from_yaml() -> YamlResult<()> {
        let yaml = r#"
---
point: [0.0, 0.0, 0.0]
"#;

        let docs = YamlLoader::load_from_str(yaml)?;
        let doc = &docs[0];

        let hash = doc.as_hash().unwrap();
        let _pt = point_from_key(hash, "point").unwrap();

        Ok(())
    }

    #[test]
    fn can_load_yaml_from_str() -> YamlResult<()> {
        let yaml_test: &str = r#"
---
- add: light
  at: [0.0, 0.0, 0.0]
  intensity: [1.0, 1.0, 1.0]

- add: sphere
  color: [0.8, 0.5, 0.0]
"#;
        let docs = YamlLoader::load_from_str(yaml_test)?;
        let _doc = &docs[0];

        Ok(())
    }
}

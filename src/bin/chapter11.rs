use rtc::io::yaml::parse_yaml;

fn main() -> anyhow::Result<()> {
    let (cam, world) = parse_yaml("samples/chapter11.yml")?;

    let canvas = cam.unwrap().render(&world, 5)?;
    canvas.export("img/chapter11.png")?;

    Ok(())
}

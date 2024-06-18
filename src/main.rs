
use tide::Request;
use aula_near_x_calc::calc_with_1;

#[async_std::main]  
async fn main() -> tide::Result<()> {
    print!("Iniciando Server...");

    let mut app: tide::Server<()> = tide::new();

    app.at("/").get(|_| async move { Ok("Hello Tide!")});
    app.at("/sum/:x/:y").get(sum);
    app.at("/sub/:x/:y").get(sub);


    print!("Servidor Iniciado...");
    print!("Acess em: http://127.0.0.1:3003");

    app.listen("127.0.0.1:3003").await?;
    Ok(())

}


async fn sum(req: Request<()>) -> tide::Result {
    let x= req.param("x").unwrap_or("0").parse().unwrap_or(0);
    let y= req.param("y").unwrap_or("0").parse().unwrap_or(0);

    let z= calc_with_1::sum_plus_one(x, y);

    Ok(format!("Resultado: {}", z).into())

}

async fn sub(req: Request<()>) -> tide::Result {

    let x= req.param("x").unwrap_or("0").parse().unwrap_or(0);
    let y= req.param("y").unwrap_or("0").parse().unwrap_or(0);

    let z= calc_with_1::sub_less_one(x, y);

    Ok(format!("Resultado: {}", z).into())

}
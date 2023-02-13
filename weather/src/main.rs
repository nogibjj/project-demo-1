use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

async fn index()->impl Responder {
    HttpResponse::Ok().body("Hello world!")
 }

#[get("/{name}")]
async fn hello_name(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {name}!"))
}


async fn get_weather(city: String) -> Result<String, Box<dyn std::error::Error>> {
    let api_key = "492d808e33b4006c459e05c300cd10da";
    let url = format!("http://api.openweathermap.org/data/2.5/weather?q={}&appid={}", city, api_key);
    let resp = reqwest::get(&url).await?;
    let weather = resp.text().await?;
    Ok(weather)
}


// get the name of the city from the url
#[get("/city/{city}")]
async fn hello_city(city: web::Path<String>) -> impl Responder {
    // get the weather from the weather api using the city name from url
    let weather = match get_weather(city.to_string()).await {
        Ok(weather) => weather,
        Err(_ee) => {
            "Error".to_owned()
        }
    };
    // return the weather

    //only show the temperature of the city
    let weather: serde_json::Value = serde_json::from_str(&weather).unwrap();
    let weather = weather["main"]["temp"].as_f64().unwrap();
    let weather = weather - 273.15;
    let weather = weather.to_string();
    // only show two decimal places
    let weather = weather.to_owned() + "." + weather.split('.').collect::<Vec<&str>>()[1].split_at(2).0;
    //set weather to renive the decimal pl 
    let weather = weather.split('.').collect::<Vec<&str>>()[0].to_owned();
    HttpResponse::Ok().body(format!("The weather in {} is {} degree in Celsius!", city, weather))
    
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Running the service");
    HttpServer::new(|| {
        App::new()
            .route("/",web::get().to(index))
            .service(hello_name)
            .service(hello_city)
        })
    .bind("127.0.0.1:8000")?
    .run()

    .await
}
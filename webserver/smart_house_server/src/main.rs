use std::sync::Mutex;
use actix_web::{get, post, delete, web, App, HttpServer, Responder, http::header::ContentType, HttpResponse};
use smart_house;
use smart_house::report::Report;
use smart_house::edifice::house::House;
use smart_house::edifice::room::Room;
use smart_house::device;
use serde::Deserialize;

#[get("/greet/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[get("/rooms")]
async fn get_rooms(house: web::Data<Mutex<House>>) -> impl Responder {
    let house = house.lock().unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body( serde_json::to_string(&house.get_rooms()).unwrap() )
}

#[get("/room/{room_name}")]
async fn get_room(house: web::Data<Mutex<House>>, room_name: web::Path<String>) -> impl Responder {
    let mut house = house.lock().unwrap();

    let room_result = house.get_room( room_name.to_string() );

    match room_result {
        Some(room) => {
            HttpResponse::Ok()
                .content_type(ContentType::json())
                .body( serde_json::to_string(&room).unwrap() )
        },
        None => {
            HttpResponse::NotFound()
                .finish()
        },
    }
}

#[post("/room/{room_name}")]
async fn add_room(house: web::Data<Mutex<House>>, room_name: web::Path<String>) -> impl Responder {
    let mut house = house.lock().unwrap();

    let room = Room {
        name: room_name.to_string(),
        devices: Vec::new(),
    };
    
    let add_result = house.add_room( room );

    match add_result {
        Ok(()) => {
            HttpResponse::Created()
                .content_type(ContentType::json())
                .finish()
        },
        Err(message) => {
            HttpResponse::InternalServerError()
                .content_type(ContentType::json())
                .body( 
                    serde_json::to_string(&message).unwrap()
                 )
        },
    }
}

#[delete("/room/{room_name}")]
async fn remove_room(house: web::Data<Mutex<House>>, room_name: web::Path<String>) -> impl Responder {
    let mut house = house.lock().unwrap();

    let remove_result = house.remove_room( room_name.to_string() );

    match remove_result {
        Ok(()) => {
            HttpResponse::Accepted()
                .content_type(ContentType::json())
                .finish()
        },
        Err(message) => {
            HttpResponse::InternalServerError()
                .content_type(ContentType::json())
                .body( 
                    serde_json::to_string(&message).unwrap()
                 )
        },
    }
}

#[get("/room/{room_name}/devices")]
async fn get_devices(house: web::Data<Mutex<House>>, room_name: web::Path<String>) -> impl Responder {
    let mut house = house.lock().unwrap();

    let room_result = house.get_room( room_name.to_string() );

    if room_result.is_none() {
        return HttpResponse::NotFound()
            .insert_header(("X-Error", "Room not found"))
            .finish();
    }

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body( serde_json::to_string(&room_result.unwrap().get_devices()).unwrap() )
}

#[derive(Clone, Deserialize)]
struct AddDevice {
    name: String,
    kind: String
}

#[post("/room/{room_name}/device")]
async fn add_device(house: web::Data<Mutex<House>>, room_name: web::Path<String>, add_device: web::Json<AddDevice>) -> impl Responder {
    let mut house = house.lock().unwrap();

    let room_result = house.get_room( room_name.to_string() );

    if room_result.is_none() {
        return HttpResponse::NotFound()
            .finish();
    }

    let device = match add_device.kind.as_str() {
        "thermometer" => Some(device::DeviceKind::Thermometer(device::thermometer::Thermometer {
            state: device::DeviceState::Off,
            temperature: 0.0,
        })),
        "electricaloutlet" => Some(device::DeviceKind::ElectricalOutlet(device::electrical_outlet::ElectricalOutlet {
            state: device::DeviceState::Off,
            power: 0.0,
        })),
        _ => None,
    };

    if device.is_none() {
        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .body( serde_json::to_string("Undefined device type").unwrap() );
    }

    let add_result = room_result.unwrap().add_device(
        add_device.name.clone(),
        device.unwrap()
    );

    match add_result {
        Ok(()) => {
            HttpResponse::Created()
                .content_type(ContentType::json())
                .finish()
        },
        Err(message) => {
            HttpResponse::InternalServerError()
                .content_type(ContentType::json())
                .body( 
                    serde_json::to_string(&message).unwrap()
                 )
        },
    }
}

#[delete("/room/{room_name}/device/{device_name}")]
async fn remove_device(house: web::Data<Mutex<House>>, path: web::Path<(String, String)>) -> impl Responder {
    let (room_name, device_name) = path.into_inner();

    let mut house = house.lock().unwrap();

    let room_result = house.get_room( room_name.to_string() );

    if room_result.is_none() {
        return HttpResponse::NotFound()
            .finish();
    }

    let remove_result = room_result.unwrap().remove_device(
        device_name.to_string()
    );

    match remove_result {
        Ok(()) => {
            HttpResponse::Ok()
                .content_type(ContentType::json())
                .finish()
        },
        Err(message) => {
            HttpResponse::InternalServerError()
                .content_type(ContentType::json())
                .body( 
                    serde_json::to_string(&message).unwrap()
                 )
        },
    }
}

#[get("/")]
async fn index(house: web::Data<Mutex<House>>) -> impl Responder {
    let house = house.lock().unwrap();
    
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body( house.get_report() )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let mutex_house = web::Data::new(Mutex::new(House {
        name: "Smart House".to_string(),
        rooms: Vec::new(),
    }));

    HttpServer::new(move || {
        App::new()
            .app_data(mutex_house.clone())
            .service(index)
            .service(greet)
            .service(get_rooms)
            .service(get_room)
            .service(add_room)
            .service(remove_room)
            .service(get_devices)
            .service(add_device)
            .service(remove_device)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
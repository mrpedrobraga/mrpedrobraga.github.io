pub mod quiz;
use rand::seq::IndexedRandom;
use rocket::{catch, catchers, get, routes, Build, Rocket};
use rocket_dyn_templates::{context, Template};

pub fn mount_routes(ro: Rocket<Build>) -> Rocket<Build> {
    ro.mount(
        "/inner-voices",
        routes![home, soundtrack, shop, resources, press_kit],
    )
    .register("/inner-voices", catchers![not_found])
}

#[get("/")]
fn home() -> Template {
    Template::render(
        "inner-voices-home",
        context! {
            title: "Home",
            path: "/",
            nav_index: 1,
        },
    )
}

#[get("/soundtrack")]
fn soundtrack() -> Template {
    Template::render(
        "inner-voices-home",
        context! {
            title: "Soundtrack",
            path: "/soundtrack",
            nav_index: 2,
        },
    )
}

#[get("/shop")]
fn shop() -> Template {
    Template::render(
        "inner-voices-home",
        context! {
            title: "Shop",
            path: "/shop",
            nav_index: 3,
        },
    )
}

#[get("/resources")]
fn resources() -> Template {
    Template::render(
        "inner-voices-home",
        context! {
            title: "Resources",
            path: "/resources",
            nav_index: 4,
        },
    )
}

#[get("/press-kit")]
fn press_kit() -> Template {
    Template::render(
        "inner-voices-home",
        context! {
            title: "Press Kit",
            path: "/press-kit",
            nav_index: 5,
        },
    )
}

#[catch(404)]
fn not_found() -> Template {
    Template::render(
        "inner-voices-not-found",
        context! {
            content: random_not_found_quip()
        },
    )
}

fn random_not_found_quip() -> &'static str {
    let options = [
        "* Aint there no such page, love...",
        "- Meow! (Took a wrong turn, human?)",
        "- Uh, I think you got something wrong...",
        "- Hang in there, you'll find your way back!",
        "- You got out of bounds, dude!",
        "- The heck did you do to end up here?",
        "- Smells like, like, the void.",
        "- ERICA!! I found a human(?) in the void!!!",
        "- So, uh, you're lost, I think.",
        "- HALT! Humans are *NOT* allowed in the void!",
        "- ZOINK! You have, it seems, gotten lost!\n- Lost, I say!",
        "- What? Playing at the 404 page? I don't pay you for that, get outta here!",
        "- Oh, dear. Rest here a while then get back home, okay?",
        "  Be not afraid. It is but a 404 page.",
        "- Not found.",
    ];
    *options.choose(&mut rand::rng()).unwrap()
}

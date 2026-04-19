pub enum Route {
    Build,
    Test,
    Patch,
    Heal,
    Idle,
}

pub fn route(input: &str) -> Route {
    if input.contains("build") {
        return Route::Build;
    }

    if input.contains("test") {
        return Route::Test;
    }

    if input.contains("error") {
        return Route::Heal;
    }

    Route::Idle
}

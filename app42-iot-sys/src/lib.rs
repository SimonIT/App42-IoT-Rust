use autocxx::include_cpp;
use ffi::*;

include_cpp! {
    #include "App42API.h"
    safety!(unsafe)
    generate!("App42API")
}

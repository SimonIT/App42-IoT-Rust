use autocxx::include_cpp;
use ffi::*;

include_cpp! {
    #include "Common/App42API.h"
    safety!(unsafe)
    generate!("App42API")
}
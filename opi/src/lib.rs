pub struct OPI5{
    led: LED,
}

struct LED {
    status: Status,
    trigger: Status,
    brightness: Status
}

enum Status {
    on,
    off 
}

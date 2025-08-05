use num

pub enum Mass<T: Float> {
    Kg(T),
    Lb(T),
    Oz(T),
}

pub enum Length<T: Float> {
    M(T),
    Km(T),
    Inch(T),
    Foot(T),
    Yard(T),
    Mile(T),
}

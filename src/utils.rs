pub fn safe_minus(one: f32, two: f32) -> f32 {
    if one >= two {
        one - two
    }else{
        two - one
    }
}

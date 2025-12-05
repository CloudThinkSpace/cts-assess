pub struct Soil {
    ph: f64,
    r#type: Option<String>,
    value: f64,
}

impl Soil {
    pub fn new(ph: f64, r#type: Option<String>, value: f64) -> Self {
        Self { ph, r#type, value }
    }

    pub fn cd_pj(&self) -> u8 {
        if self.ph <= 5.5 {
            if self.r#type == Some("QT".to_string()) || self.r#type.is_none() {
                if self.value <= 0.3 {
                    1
                } else if self.value > 0.3 && self.value <= 1.5 {
                    2
                } else {
                    3
                }
            } else if self.value <= 0.3 {
                1
            } else if self.value > 0.3 && self.value <= 1.5 {
                2
            } else {
                3
            }
        } else if self.ph > 5.5 && self.ph <= 6.5 {
            if self.r#type == Some("QT".to_string()) || self.r#type.is_none() {
                if self.value <= 0.3 {
                    1
                } else if self.value > 0.3 && self.value <= 2.0 {
                    2
                } else {
                    3
                }
            } else if self.value <= 0.4 {
                1
            } else if self.value > 0.4 && self.value <= 2.0 {
                2
            } else {
                3
            }
        } else if self.ph > 6.5 && self.ph <= 7.5 {
            if self.r#type == Some("QT".to_string()) || self.r#type.is_none() {
                if self.value <= 0.3 {
                    1
                } else if self.value > 0.3 && self.value <= 3.0 {
                    2
                } else {
                    3
                }
            } else if self.value <= 0.6 {
                1
            } else if self.value > 0.6 && self.value <= 3.0 {
                2
            } else {
                3
            }
        } else if self.r#type == Some("QT".to_string()) || self.r#type.is_none() {
            if self.value <= 0.6 {
                1
            } else if self.value > 0.6 && self.value <= 4.0 {
                2
            } else {
                3
            }
        } else if self.value <= 0.8 {
            1
        } else if self.value > 0.8 && self.value <= 4.0 {
            2
        } else {
            3
        }
    }

    pub fn hg_pj(&self) -> u8 {
        if self.ph <= 5.5 {
            if self.r#type == Some("QT".to_string()) || self.r#type.is_none() {
                if self.value <= 1.3 {
                    1
                } else if self.value > 1.3 && self.value <= 2.0 {
                    2
                } else {
                    3
                }
            } else if self.value <= 0.5 {
                1
            } else if self.value > 0.5 && self.value <= 2.0 {
                2
            } else {
                3
            }
        } else if self.ph > 5.5 && self.ph <= 6.5 {
            if self.r#type == Some("QT".to_string()) || self.r#type.is_none() {
                if self.value <= 1.8 {
                    1
                } else if self.value > 1.8 && self.value <= 2.5 {
                    2
                } else {
                    3
                }
            } else if self.value <= 0.5 {
                1
            } else if self.value > 0.5 && self.value <= 2.5 {
                2
            } else {
                3
            }
        } else if self.ph > 6.5 && self.ph <= 7.5 {
            if self.r#type == Some("QT".to_string()) || self.r#type.is_none() {
                if self.value <= 2.4 {
                    1
                } else if self.value > 2.4 && self.value <= 4.0 {
                    2
                } else {
                    3
                }
            } else if self.value <= 0.6 {
                1
            } else if self.value > 0.6 && self.value <= 4.0 {
                2
            } else {
                3
            }
        } else if self.r#type == Some("QT".to_string()) || self.r#type.is_none() {
            if self.value <= 3.4 {
                1
            } else if self.value > 3.4 && self.value <= 6.0 {
                2
            } else {
                3
            }
        } else if self.value <= 1.0 {
            1
        } else if self.value > 1.0 && self.value <= 6.0 {
            2
        } else {
            3
        }
    }

    pub fn as_pj(&self) -> u8 {
        if self.ph <= 5.5 {
            if self.r#type == Some("QT".to_string()) || self.r#type.is_none() {
                if self.value <= 40.0 {
                    1
                } else if self.value > 40.0 && self.value <= 200.0 {
                    2
                } else {
                    3
                }
            } else if self.value <= 30.0 {
                1
            } else if self.value > 30.0 && self.value <= 200.0 {
                2
            } else {
                3
            }
        } else if self.ph > 5.5 && self.ph <= 6.5 {
            if self.r#type == Some("QT".to_string()) || self.r#type.is_none() {
                if self.value <= 40.0 {
                    1
                } else if self.value > 40.0 && self.value <= 150.0 {
                    2
                } else {
                    3
                }
            } else if self.value <= 30.0 {
                1
            } else if self.value > 30.0 && self.value <= 150.0 {
                2
            } else {
                3
            }
        } else if self.ph > 6.5 && self.ph <= 7.5 {
            if self.r#type == Some("QT".to_string()) || self.r#type.is_none() {
                if self.value <= 30.0 {
                    1
                } else if self.value > 30.0 && self.value <= 120.0 {
                    2
                } else {
                    3
                }
            } else if self.value <= 25.0 {
                1
            } else if self.value > 25.0 && self.value <= 120.0 {
                2
            } else {
                3
            }
        } else if self.r#type == Some("QT".to_string()) || self.r#type.is_none() {
            if self.value <= 25.0 {
                1
            } else if self.value > 25.0 && self.value <= 100.0 {
                2
            } else {
                3
            }
        } else if self.value <= 20.0 {
            1
        } else if self.value > 20.0 && self.value <= 100.0 {
            2
        } else {
            3
        }
    }

    pub fn pb_pj(&self) -> u8 {
        if self.ph <= 5.5 {
            if self.r#type == Some("QT".to_string()) || self.r#type.is_none() {
                if self.value <= 70.0 {
                    1
                } else if self.value > 70.0 && self.value <= 400.0 {
                    2
                } else {
                    3
                }
            } else if self.value <= 80.0 {
                1
            } else if self.value > 80.0 && self.value <= 400.0 {
                2
            } else {
                3
            }
        } else if self.ph > 5.5 && self.ph <= 6.5 {
            if self.r#type == Some("QT".to_string()) || self.r#type.is_none() {
                if self.value <= 90.0 {
                    1
                } else if self.value > 90.0 && self.value <= 500.0 {
                    2
                } else {
                    3
                }
            } else if self.value <= 100.0 {
                1
            } else if self.value > 100.0 && self.value <= 500.0 {
                2
            } else {
                3
            }
        } else if self.ph > 6.5 && self.ph <= 7.5 {
            if self.r#type == Some("QT".to_string()) || self.r#type.is_none() {
                if self.value <= 120.0 {
                    1
                } else if self.value > 120.0 && self.value <= 700.0 {
                    2
                } else {
                    3
                }
            } else if self.value <= 140.0 {
                1
            } else if self.value > 140.0 && self.value <= 700.0 {
                2
            } else {
                3
            }
        } else if self.r#type == Some("QT".to_string()) || self.r#type.is_none() {
            if self.value <= 170.0 {
                1
            } else if self.value > 170.0 && self.value <= 1000.0 {
                2
            } else {
                3
            }
        } else if self.value <= 240.0 {
            1
        } else if self.value > 240.0 && self.value <= 1000.0 {
            2
        } else {
            3
        }
    }

    pub fn cr_pj(&self) -> u8 {
        if self.ph <= 5.5 {
            if self.r#type == Some("QT".to_string()) || self.r#type.is_none() {
                if self.value <= 150.0 {
                    1
                } else if self.value > 150.0 && self.value <= 800.0 {
                    2
                } else {
                    3
                }
            } else if self.value <= 250.0 {
                1
            } else if self.value > 250.0 && self.value <= 800.0 {
                2
            } else {
                3
            }
        } else if self.ph > 5.5 && self.ph <= 6.5 {
            if self.r#type == Some("QT".to_string()) || self.r#type.is_none() {
                if self.value <= 150.0 {
                    1
                } else if self.value > 150.0 && self.value <= 850.0 {
                    2
                } else {
                    3
                }
            } else if self.value <= 250.0 {
                1
            } else if self.value > 250.0 && self.value <= 850.0 {
                2
            } else {
                3
            }
        } else if self.ph > 6.5 && self.ph <= 7.5 {
            if self.r#type == Some("QT".to_string()) || self.r#type.is_none() {
                if self.value <= 200.0 {
                    1
                } else if self.value > 200.0 && self.value <= 1000.0 {
                    2
                } else {
                    3
                }
            } else if self.value <= 300.0 {
                1
            } else if self.value > 300.0 && self.value <= 1000.0 {
                2
            } else {
                3
            }
        } else if self.r#type == Some("QT".to_string()) || self.r#type.is_none() {
            if self.value <= 250.0 {
                1
            } else if self.value > 250.0 && self.value <= 1300.0 {
                2
            } else {
                3
            }
        } else if self.value <= 350.0 {
            1
        } else if self.value > 350.0 && self.value <= 1300.0 {
            2
        } else {
            3
        }
    }

    pub fn cu_pj(&self) -> u8 {
        if self.ph <= 6.5 {
            if self.r#type == Some("QT".to_string()) || self.r#type.is_none() {
                if self.value <= 50.0 { 1 } else { 2 }
            } else if self.value <= 150.0 {
                1
            } else {
                2
            }
        } else if self.ph > 6.5 && self.ph <= 7.5 {
            if self.r#type == Some("QT".to_string()) || self.r#type.is_none() {
                if self.value <= 100.0 { 1 } else { 2 }
            } else if self.value <= 200.0 {
                1
            } else {
                2
            }
        } else if self.r#type == Some("QT".to_string()) || self.r#type.is_none() {
            if self.value <= 100.0 { 1 } else { 2 }
        } else if self.value <= 200.0 {
            1
        } else {
            2
        }
    }

    pub fn ni_pj(&self) -> u8 {
        if self.ph <= 5.5 {
            if self.value <= 60.0 { 1 } else { 2 }
        } else if self.ph > 5.5 && self.ph <= 6.5 {
            if self.value <= 70.0 { 1 } else { 2 }
        } else if self.ph > 6.5 && self.ph <= 7.5 {
            if self.value <= 100.0 { 1 } else { 2 }
        } else if self.value <= 190.0 {
            1
        } else {
            2
        }
    }

    pub fn zn_pj(&self) -> u8 {
        if self.ph <= 6.5 {
            if self.value <= 200.0 { 1 } else { 2 }
        } else if self.ph > 6.5 && self.ph <= 7.5 {
            if self.value <= 250.0 { 1 } else { 2 }
        } else if self.value <= 300.0 {
            1
        } else {
            2
        }
    }
}

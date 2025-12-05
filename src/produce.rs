pub struct Produce {
    name: String,
    value: f64,
}

impl Produce {
    pub fn new(name: &str, value: f64) -> Self {
        Self {
            name: name.to_string(),
            value,
        }
    }

    pub fn cd_pj(&self) -> u8 {
        if ["GW", "JL", "DLSC", "KGJSL"].contains(&self.name.as_str()) {
            if self.value <= 0.1 { 1 } else { 2 }
        } else if ["SD", "YCL", "QTSC", "DL"].contains(&self.name.as_str()) {
            if self.value <= 0.2 { 1 } else { 2 }
        } else if ["LJL", "GQL", "YTL", "SG", "GLSG", "JGXLSG"].contains(&self.name.as_str()) {
            if self.value <= 0.05 { 1 } else { 2 }
        } else if ["JG"].contains(&self.name.as_str()) {
            if self.value <= 0.5 { 1 } else { 2 }
        } else if ["SICAO"].contains(&self.name.as_str()) {
            if self.value <= 1.0 { 1 } else { 2 }
        } else {
            1
        }
    }

    pub fn hg_pj(&self) -> u8 {
        if ["GW", "SD"].contains(&self.name.as_str()) {
            if self.value <= 0.02 { 1 } else { 2 }
        } else if ["LJL", "GQL", "YTL", "JL", "DLSC", "KGJSL", "YCL", "QTSC"]
            .contains(&self.name.as_str())
        {
            if self.value <= 0.01 { 1 } else { 2 }
        } else if ["SICAO"].contains(&self.name.as_str()) {
            if self.value <= 0.1 { 1 } else { 2 }
        } else {
            1
        }
    }

    pub fn as_pj(&self) -> u8 {
        if [
            "GW", "SD", "LJL", "GQL", "YTL", "JL", "DLSC", "KGJSL", "YCL", "QTSC",
        ]
        .contains(&self.name.as_str())
        {
            if self.value <= 0.5 { 1 } else { 2 }
        } else if ["SICAO"].contains(&self.name.as_str()) {
            if self.value <= 4.0 { 1 } else { 2 }
        } else {
            1
        }
    }

    pub fn pb_pj(&self) -> u8 {
        if ["GW", "SD", "DLSC", "KGJSL", "DL", "JGXLSG", "JG"].contains(&self.name.as_str()) {
            if self.value <= 0.2 { 1 } else { 2 }
        } else if ["LJL", "GQL", "JL", "QTSC", "SG", "GLSG"].contains(&self.name.as_str()) {
            if self.value <= 0.1 { 1 } else { 2 }
        } else if ["YTL", "YCL"].contains(&self.name.as_str()) {
            if self.value <= 0.3 { 1 } else { 2 }
        } else if ["SICAO"].contains(&self.name.as_str()) {
            if self.value <= 30.0 { 1 } else { 2 }
        } else {
            1
        }
    }

    pub fn cr_pj(&self) -> u8 {
        if ["GW", "SD", "DL"].contains(&self.name.as_str()) {
            if self.value <= 1.0 { 1 } else { 2 }
        } else if ["LJL", "GQL", "YTL", "JL", "DLSC", "KGJSL", "YCL", "QTSC"]
            .contains(&self.name.as_str())
        {
            if self.value <= 0.5 { 1 } else { 2 }
        } else if ["SICAO"].contains(&self.name.as_str()) {
            if self.value <= 5.0 { 1 } else { 2 }
        } else {
            1
        }
    }
}

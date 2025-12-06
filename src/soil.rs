use crate::heavy_metal::{HeavyEnum, HeavyMetal};

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

pub fn soil_pj(
    data: &[Vec<String>],
    heavy_metals: &[HeavyMetal],
    ph_index: usize,
    nydlx_index: Option<usize>,
) -> Result<Vec<Vec<String>>, String> {
    // 判断数据是否为空
    if data.len() <= 1 {
        return Err("无法计算，数据表为空".to_string());
    }
    // 第一行数据是表头
    let mut row_header: Vec<String> = data
        .first()
        .unwrap()
        .iter()
        .map(|item| item.to_string())
        .collect();
    // 添加重金属评价字段
    for heavy_metal in heavy_metals {
        // 重金属名称
        let name = &heavy_metal.name;
        // 添加表头数据
        row_header.push(name.to_string());
    }
    // 添加3项重金属综合评价表头
    row_header.push("3项综合评价".to_string());
    // 添加5项重金属综合评价表头
    row_header.push("5项综合评价".to_string());
    // 添加8项重金属综合评价表头
    row_header.push("3项污染物汇总".to_string());
    // 添加超标表头
    row_header.push("5项污染物汇总".to_string());
    // 数据收集
    let mut result = Vec::new();
    // 遍历每条数据
    for (index, row) in data.iter().skip(1).enumerate() {
        // 每行数据
        let mut row_result = Vec::new();
        for cell in row.iter() {
            row_result.push(cell.to_string());
        }
        let mut heavy_3 = Vec::new();
        let mut heavy_5 = Vec::new();
        let mut class3: u8 = 0;
        let mut class5: u8 = 0;
        // 遍历重金属
        for heavy_metal in heavy_metals {
            // 重金属名称
            let name = &heavy_metal.name;
            // 重金属位置
            let heavy_metal_index = heavy_metal.index;
            // 重金属值
            let value = row[heavy_metal_index]
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>()
                .parse::<f64>()
                .map_err(|_| {
                    format!(
                        "第{}行，重金属【{}】的值【{}】无效。",
                        index + 1,
                        name,
                        &row[heavy_metal_index]
                    )
                })?;
            // ph值
            let ph_value = row[ph_index]
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>()
                .parse::<f64>()
                .map_err(|_| format!("第{}行，PH的值【{}】无效。", index + 1, &row[ph_index]))?;

            // 农用地类型
            let nydlx_value = if let Some(ny_index) = nydlx_index {
                &row[ny_index]
            } else {
                "QT"
            };
            let soil = Soil::new(ph_value, Some(nydlx_value.to_string()), value);
            // 进行评级
            let pj_value = match heavy_metal.code {
                HeavyEnum::Cd => {
                    let value = soil.cd_pj();
                    if class5 < value {
                        class5 = value
                    }
                    if value > 1 {
                        heavy_5.push(heavy_metal.code.to_string());
                    }
                    value
                }
                HeavyEnum::Hg => {
                    let value = soil.hg_pj();
                    if class5 < value {
                        class5 = value
                    }
                    if value > 1 {
                        heavy_5.push(heavy_metal.code.to_string());
                    }
                    value
                }
                HeavyEnum::As => {
                    let value = soil.as_pj();
                    if class5 < value {
                        class5 = value
                    }
                    if value > 1 {
                        heavy_5.push(heavy_metal.code.to_string());
                    }
                    value
                }
                HeavyEnum::Pb => {
                    let value = soil.pb_pj();
                    if class5 < value {
                        class5 = value
                    }
                    if value > 1 {
                        heavy_5.push(heavy_metal.code.to_string());
                    }
                    value
                }
                HeavyEnum::Cr => {
                    let value = soil.cr_pj();
                    if class5 < value {
                        class5 = value
                    }
                    if value > 1 {
                        heavy_5.push(heavy_metal.code.to_string());
                    }
                    value
                }
                HeavyEnum::Cu => {
                    let value = soil.cu_pj();
                    if class3 < value {
                        class3 = value
                    }
                    if value > 1 {
                        heavy_3.push(heavy_metal.code.to_string());
                    }
                    value
                }
                HeavyEnum::Ni => {
                    let value = soil.ni_pj();
                    if class3 < value {
                        class3 = value
                    }
                    if value > 1 {
                        heavy_3.push(heavy_metal.code.to_string());
                    }
                    value
                }
                HeavyEnum::Zn => {
                    let value = soil.zn_pj();
                    if class3 < value {
                        class3 = value
                    }
                    if value > 1 {
                        heavy_3.push(heavy_metal.code.to_string());
                    }
                    value
                }
            };
            // 收集结果
            row_result.push(pj_value.to_string());
        }

        // 添加3项重金属综合评价数据
        row_result.push(class3.to_string());
        // 添加5项重金属综合评价数据
        row_result.push(class5.to_string());

        row_result.push(heavy_3.join(","));
        // 添加汇总重金属
        row_result.push(heavy_5.join(","));
        // 收集数据
        result.push(row_result);
    }
    // 插入表头
    result.insert(0, row_header);

    Ok(result)
}

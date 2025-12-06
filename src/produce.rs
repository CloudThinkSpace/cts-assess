use crate::heavy_metal::{HeavyEnum, HeavyMetal};

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

pub fn produce_pj(
    data: &[Vec<String>],
    heavy_metals: &[HeavyMetal],
    ncp_index: usize,
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
    // 添加综合评价表头
    row_header.push("综合评价".to_string());
    // 添加超标表头
    row_header.push("污染物汇总".to_string());
    // 数据收集
    let mut result = Vec::new();

    // 遍历每条数据
    for (index, row) in data.iter().skip(1).enumerate() {
        // 每行数据
        let mut row_result = Vec::new();
        for cell in row.iter() {
            row_result.push(cell.to_string());
        }
        // 超标元素
        let mut heavy_elements = Vec::new();
        let mut comprehensiveness: u8 = 1;
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

            // 农产品
            let ncp_value = &row[ncp_index];
            let produce = Produce::new(ncp_value, value);
            // 进行评级
            let pj_value = match heavy_metal.code {
                HeavyEnum::Cd => produce.cd_pj(),
                HeavyEnum::Hg => produce.hg_pj(),
                HeavyEnum::As => produce.as_pj(),
                HeavyEnum::Pb => produce.pb_pj(),
                HeavyEnum::Cr => produce.cr_pj(),
                _ => 1,
            };
            // 收集结果
            row_result.push(pj_value.to_string());

            // 判断综合评价级别
            if comprehensiveness < pj_value {
                comprehensiveness = pj_value
            }
            // 判断是否超标
            if pj_value > 1 {
                heavy_elements.push(heavy_metal.code.to_string());
            }
        }

        // 添加综合评价数据
        row_result.push(comprehensiveness.to_string());
        // 添加汇总重金属
        row_result.push(heavy_elements.join(","));
        // 收集数据
        result.push(row_result);
    }
    // 插入表头
    result.insert(0, row_header);
    Ok(result)
}

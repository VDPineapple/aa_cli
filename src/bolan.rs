use std::collections::HashMap;

pub fn get_bolan() -> HashMap<String, HashMap<usize, f32>> {
    let mut bolan = HashMap::new();
    let mut swamp_clay_rds = HashMap::new();
    swamp_clay_rds.insert(3, 0.374988);
    swamp_clay_rds.insert(4, 0.250116);
    swamp_clay_rds.insert(1, 0.187387);
    bolan.insert("SWAMP_CLAY REDSTONE".to_string(), swamp_clay_rds);
    let mut swamp_clay_dmd = HashMap::new();
    swamp_clay_dmd.insert(8, 0.500088);
    swamp_clay_dmd.insert(6, 0.3749);
    bolan.insert("SWAMP_CLAY DIAMOND".to_string(), swamp_clay_dmd);
    let mut swamp_clay = HashMap::new();
    swamp_clay.insert(11, 0.500081);
    swamp_clay.insert(12, 0.249968);
    swamp_clay.insert(10, 0.187493);
    bolan.insert("SWAMP_CLAY LAPIS".to_string(), swamp_clay);
    let mut river_clay_rds = HashMap::new();
    river_clay_rds.insert(14, 0.75008);
    river_clay_rds.insert(8, 0.125033);
    river_clay_rds.insert(5, 0.093684);
    bolan.insert("RIVER_CLAY REDSTONE".to_string(), river_clay_rds);
    let mut river_clay_dmd = HashMap::new();
    river_clay_dmd.insert(3, 0.374965);
    river_clay_dmd.insert(4, 0.249936);
    river_clay_dmd.insert(1, 0.187548);
    bolan.insert("RIVER_CLAY DIAMOND".to_string(), river_clay_dmd);
    let mut river_clay = HashMap::new();
    river_clay.insert(8, 0.499903);
    river_clay.insert(6, 0.375183);
    bolan.insert("RIVER_CLAY LAPIS".to_string(), river_clay);
    let mut gravel_rds = HashMap::new();
    gravel_rds.insert(10, 0.374878);
    gravel_rds.insert(11, 0.187476);
    gravel_rds.insert(9, 0.125008);
    bolan.insert("GRAVEL REDSTONE".to_string(), gravel_rds);
    let mut gravel_dmd = HashMap::new();
    gravel_dmd.insert(14, 0.74992);
    gravel_dmd.insert(8, 0.125093);
    gravel_dmd.insert(5, 0.093641);
    bolan.insert("GRAVEL DIAMOND".to_string(), gravel_dmd);
    let mut gravel = HashMap::new();
    gravel.insert(3, 0.375061);
    gravel.insert(4, 0.250105);
    gravel.insert(1, 0.187341);
    bolan.insert("GRAVEL LAPIS".to_string(), gravel);

    bolan

}
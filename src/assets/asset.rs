use std::collections::HashMap;

pub struct Assets {
    asset_types: HashMap< String, AssetType>
}

pub struct AssetPolygon {
    pub color: (f32, f32, f32, f32),
    pub vertices: Vec< (f32, f32) >,
    pub drawlist: Vec< u32 >
}

pub struct AssetType {
    cur_asset: usize,
    assets: HashMap<String, Asset>
}

pub struct Asset {
    polygons: Vec< AssetPolygon >
}

impl Assets {
    pub fn new() -> Assets {
        let assets = HashMap::new();
        Assets { asset_types: assets }
    }

    pub fn add_asset(&mut self, 
                     asset_type: String, 
                     asset_subtype: String, 
                     asset: Asset) {
        if self.asset_types.contains_key(&asset_type) == false {
            self.asset_types.insert(asset_type.clone(), AssetType::new());
        }
        self.asset_types.get_mut(&asset_type).unwrap().add_asset(asset, asset_subtype);
    }
    
    pub fn get_asset(&mut self, asset_type: &String, asset_subtype: &String) -> &mut Asset {
        return self.asset_types.get_mut(asset_type).unwrap().assets.get_mut(asset_subtype).unwrap();
    }
        
}
impl AssetPolygon {
    pub fn new(color: (f32, f32, f32, f32)) -> AssetPolygon {
        let vertices = Vec::new();
        let drawlist = Vec::new();
        AssetPolygon { color: color,
                       vertices: vertices,
                       drawlist: drawlist }
    }
    pub fn add_vertex(&mut self, vertex: (f32, f32) ) {
        self.vertices.push(vertex);
    }

    pub fn add_index(&mut self, index: u32 ) {
        self.drawlist.push(index);
    }
}
impl AssetType {
    pub fn new() -> AssetType {
        let assets = HashMap::new();

        AssetType { cur_asset:      0,
                    assets:         assets }
    }

    pub fn add_asset(&mut self, asset: Asset, id: String) {
        self.assets.insert(id, asset);
    }
}

impl Asset {
    pub fn new() -> Asset {
        let polygons = Vec::new();
        
        Asset { polygons: polygons }
        }
    pub fn num_polies(&mut self) -> usize {
        return self.polygons.len()
    }

    pub fn get_poly(&mut self, index: usize) -> &AssetPolygon{
        return &self.polygons[index];
    }

    pub fn add_polygon(&mut self, poly: AssetPolygon) {
        self.polygons.push(poly);
    }
}


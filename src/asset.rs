use std::collections::HashMap;

struct AssetPolygon {
    color: (f32, f32, f32),
    vertices: Vec< (f32, f32) >,
    drawlist: Vec< u32 >
}

struct AssetType {
    cur_asset: usize,
    assets: HashMap<String, Asset>
}

struct Asset {
    polygons: Vec< AssetPolygon >
}

impl AssetType {
    fn new() -> AssetType {
        let assets = HashMap::new();

        AssetType { cur_asset:      0,
                    assets:         assets }
    }

    fn add_asset(&mut self, asset: Asset, id: String) {
        self.assets.insert(id, asset);
    }
}

impl Asset {
    fn new() -> Asset {
        let mut polygons = Vec::new();
        
        Asset { polygons: polygons
        }
    }
}


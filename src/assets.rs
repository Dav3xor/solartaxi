
use std::collections::HashMap;

pub mod asset;


pub fn build_assets() -> asset::Assets {
    let mut assets = asset::Assets::new();

    let mut asset = asset::Asset::new();
    let mut poly = asset::AssetPolygon::new((0.2823529411764706,
                                             0.5294117647058824,
                                             0.0392156862745098) );
    poly.add_vertex((-0.26458333, -0.0));
    poly.add_vertex((-0.26458333, 2.910419999999988));
    poly.add_vertex((6.6145835, 2.910419999999988));
    poly.add_vertex((6.6145835, 2.116669999999999));
    poly.add_vertex((9.392708500000001, 0.6614599999999768));
    poly.add_vertex((10.186458000000002, -0.6614600000000337));
    poly.add_vertex((3.042708300000002, -0.6614600000000337));
    poly.add_vertex((3.836458300000002, 0.6614599999999768));
    poly.add_vertex((6.350000200000002, 2.116669999999999));
    poly.add_vertex((6.350000200000002, 2.6458299999999895));
    poly.add_vertex((0.26458333000000156, 2.6458299999999895));
    poly.add_vertex((0.26458333000000156, -0.0));

    poly.add_index(0); poly.add_index(11); poly.add_index(10);
    poly.add_index(8); poly.add_index(7); poly.add_index(6);
    poly.add_index(8); poly.add_index(6); poly.add_index(5);
    poly.add_index(8); poly.add_index(5); poly.add_index(4);
    poly.add_index(8); poly.add_index(4); poly.add_index(3);
    poly.add_index(1); poly.add_index(0); poly.add_index(10);
    poly.add_index(9); poly.add_index(8); poly.add_index(3);
    poly.add_index(9); poly.add_index(3); poly.add_index(2);
    poly.add_index(2); poly.add_index(1); poly.add_index(10);
    poly.add_index(2); poly.add_index(10); poly.add_index(9);
    asset.add_polygon(poly);

    assets.add_asset("lamp".to_string(), 
                     "4".to_string(), 
                     asset);
    
    


    let mut asset = asset::Asset::new();
    let mut poly = asset::AssetPolygon::new((0.45098039215686275,
                                             0.9137254901960784,
                                             0.011764705882352941) );
    poly.add_vertex((2.7781248, -0.0));
    poly.add_vertex((2.9104165, 0.1322900000000118));
    poly.add_vertex((2.9104165, 0.9260400000000004));
    poly.add_vertex((2.7781248, 1.058339999999987));
    poly.add_vertex((2.3812499000000003, 1.058339999999987));
    poly.add_vertex((2.3812499000000003, 8.202079999999967));
    poly.add_vertex((3.1749999000000004, 8.731249999999989));
    poly.add_vertex((2.9104166000000005, 10.186460000000011));
    poly.add_vertex((-6.0000001e-08, 10.58332999999999));
    poly.add_vertex((-2.9104168, 10.18644999999998));
    poly.add_vertex((-3.1750001, 8.731249999999989));
    poly.add_vertex((-2.3812501, 8.202079999999967));
    poly.add_vertex((-2.3812501, 1.058339999999987));
    poly.add_vertex((-2.7781251, 1.058339999999987));
    poly.add_vertex((-2.9104168, 0.9260400000000004));
    poly.add_vertex((-2.9104168, 0.1322900000000118));
    poly.add_vertex((-2.7781251, -0.0));

    poly.add_index(16); poly.add_index(0); poly.add_index(1);
    poly.add_index(16); poly.add_index(1); poly.add_index(2);
    poly.add_index(16); poly.add_index(2); poly.add_index(3);
    poly.add_index(16); poly.add_index(3); poly.add_index(4);
    poly.add_index(5); poly.add_index(6); poly.add_index(7);
    poly.add_index(5); poly.add_index(7); poly.add_index(8);
    poly.add_index(5); poly.add_index(8); poly.add_index(9);
    poly.add_index(5); poly.add_index(9); poly.add_index(10);
    poly.add_index(5); poly.add_index(10); poly.add_index(11);
    poly.add_index(12); poly.add_index(13); poly.add_index(14);
    poly.add_index(12); poly.add_index(14); poly.add_index(15);
    poly.add_index(12); poly.add_index(15); poly.add_index(16);
    poly.add_index(12); poly.add_index(16); poly.add_index(4);
    poly.add_index(12); poly.add_index(4); poly.add_index(5);
    poly.add_index(12); poly.add_index(5); poly.add_index(11);
    asset.add_polygon(poly);

    let mut poly = asset::AssetPolygon::new((0.1607843137254902,
                                             0.1411764705882353,
                                             0.0196078431372549) );
    poly.add_vertex((-6.0000001e-08, 9.921870000000013));
    poly.add_vertex((1.7197915, 9.524999999999977));
    poly.add_vertex((1.8520832, 8.598950000000002));
    poly.add_vertex((-6.0000001e-08, 8.46665999999999));
    poly.add_vertex((-1.8520834, 8.598950000000002));
    poly.add_vertex((-1.5875001, 9.524999999999977));

    poly.add_index(0); poly.add_index(5); poly.add_index(4);
    poly.add_index(0); poly.add_index(4); poly.add_index(3);
    poly.add_index(0); poly.add_index(3); poly.add_index(2);
    poly.add_index(0); poly.add_index(2); poly.add_index(1);
    asset.add_polygon(poly);

    assets.add_asset("wastebin".to_string(), 
                     "1".to_string(), 
                     asset);
    
    


    let mut asset = asset::Asset::new();
    let mut poly = asset::AssetPolygon::new((0.5098039215686274,
                                             0.3333333333333333,
                                             0.0196078431372549) );
    poly.add_vertex((1.3229166, -0.0));
    poly.add_vertex((1.3229166, 0.5291700000000219));
    poly.add_vertex((0.7937500200000001, 0.5291700000000219));
    poly.add_vertex((0.7937500200000001, 1.8520800000000008));
    poly.add_vertex((1.0583333000000001, 2.116669999999999));
    poly.add_vertex((1.0583333000000001, 2.910419999999988));
    poly.add_vertex((0.7937500200000002, 3.1750000000000114));
    poly.add_vertex((0.7937501000000001, 4.101040000000012));
    poly.add_vertex((1.0583334000000002, 4.101040000000012));
    poly.add_vertex((1.0583333000000001, 4.49790999999999));
    poly.add_vertex((0.7937500200000002, 4.49790999999999));
    poly.add_vertex((0.7937500500000002, 4.762490000000014));
    poly.add_vertex((0.6614584100000003, 5.027080000000012));
    poly.add_vertex((0.2645833400000003, 5.291660000000036));
    poly.add_vertex((0.2645833400000003, 5.556250000000034));
    poly.add_vertex((-0.2645832299999997, 5.556250000000034));
    poly.add_vertex((-0.2645832299999997, 5.291660000000036));
    poly.add_vertex((-0.6614582299999997, 5.027080000000012));
    poly.add_vertex((-0.7937499299999997, 4.762490000000014));
    poly.add_vertex((-0.7937499299999997, 4.49790999999999));
    poly.add_vertex((-1.0583332, 4.49790999999999));
    poly.add_vertex((-1.0583331, 4.101040000000012));
    poly.add_vertex((-0.79374983, 4.101040000000012));
    poly.add_vertex((-0.7937499299999999, 3.1750000000000114));
    poly.add_vertex((-1.0583331999999999, 2.910419999999988));
    poly.add_vertex((-1.0583331999999999, 2.116669999999999));
    poly.add_vertex((-0.7937499299999999, 1.8520800000000008));
    poly.add_vertex((-0.7937499299999999, 0.5291700000000219));
    poly.add_vertex((-1.3229165, 0.5291700000000219));
    poly.add_vertex((-1.3229165, -0.0));

    poly.add_index(29); poly.add_index(0); poly.add_index(1);
    poly.add_index(29); poly.add_index(1); poly.add_index(2);
    poly.add_index(3); poly.add_index(4); poly.add_index(5);
    poly.add_index(3); poly.add_index(5); poly.add_index(6);
    poly.add_index(7); poly.add_index(8); poly.add_index(9);
    poly.add_index(7); poly.add_index(9); poly.add_index(10);
    poly.add_index(10); poly.add_index(11); poly.add_index(12);
    poly.add_index(10); poly.add_index(12); poly.add_index(13);
    poly.add_index(13); poly.add_index(14); poly.add_index(15);
    poly.add_index(13); poly.add_index(15); poly.add_index(16);
    poly.add_index(16); poly.add_index(17); poly.add_index(18);
    poly.add_index(16); poly.add_index(18); poly.add_index(19);
    poly.add_index(19); poly.add_index(20); poly.add_index(21);
    poly.add_index(19); poly.add_index(21); poly.add_index(22);
    poly.add_index(23); poly.add_index(24); poly.add_index(25);
    poly.add_index(23); poly.add_index(25); poly.add_index(26);
    poly.add_index(27); poly.add_index(28); poly.add_index(29);
    poly.add_index(29); poly.add_index(2); poly.add_index(3);
    poly.add_index(6); poly.add_index(7); poly.add_index(10);
    poly.add_index(6); poly.add_index(10); poly.add_index(13);
    poly.add_index(6); poly.add_index(13); poly.add_index(16);
    poly.add_index(6); poly.add_index(16); poly.add_index(19);
    poly.add_index(6); poly.add_index(19); poly.add_index(22);
    poly.add_index(22); poly.add_index(23); poly.add_index(26);
    poly.add_index(27); poly.add_index(29); poly.add_index(3);
    poly.add_index(3); poly.add_index(6); poly.add_index(22);
    poly.add_index(3); poly.add_index(22); poly.add_index(26);
    poly.add_index(3); poly.add_index(26); poly.add_index(27);
    asset.add_polygon(poly);

    assets.add_asset("hydrant".to_string(), 
                     "1".to_string(), 
                     asset);
    
    


    let mut asset = asset::Asset::new();
    let mut poly = asset::AssetPolygon::new((0.2823529411764706,
                                             0.5294117647058824,
                                             0.0392156862745098) );
    poly.add_vertex((0.79375002, -0.0));
    poly.add_vertex((0.79375002, 0.5291700000000219));
    poly.add_vertex((1.0583333, 1.3229200000000105));
    poly.add_vertex((1.8520834, 2.3812500000000227));
    poly.add_vertex((2.6458333, 2.6458300000000463));
    poly.add_vertex((3.9687499, 2.9104200000000446));
    poly.add_vertex((3.9687499, 3.439580000000035));
    poly.add_vertex((4.2333333, 3.439580000000035));
    poly.add_vertex((4.2333333, 3.968750000000057));
    poly.add_vertex((5.0270833, 4.23333000000008));
    poly.add_vertex((5.8208333, 7.67292000000009));
    poly.add_vertex((3.7041666, 8.466670000000079));
    poly.add_vertex((3.4395833000000002, 9.260420000000067));
    poly.add_vertex((3.1750000000000003, 8.466670000000079));
    poly.add_vertex((1.0583333000000001, 7.67292000000009));
    poly.add_vertex((1.8520834000000002, 4.23333000000008));
    poly.add_vertex((2.6458333, 3.968750000000057));
    poly.add_vertex((2.6458333, 3.439580000000035));
    poly.add_vertex((2.9104166, 3.439580000000035));
    poly.add_vertex((2.9104166, 3.1750000000000114));
    poly.add_vertex((1.3229167, 2.910419999999988));
    poly.add_vertex((0.5291666599999999, 2.381249999999966));
    poly.add_vertex((-0.0, 1.5874999999999773));
    poly.add_vertex((-0.52916666, 2.3812500000000227));
    poly.add_vertex((-1.3229167, 2.910419999999988));
    poly.add_vertex((-2.9104166, 3.1750000000000114));
    poly.add_vertex((-2.9104166, 3.439580000000035));
    poly.add_vertex((-2.6458333, 3.439580000000035));
    poly.add_vertex((-2.6458333, 3.968750000000057));
    poly.add_vertex((-1.8520834, 4.233330000000024));
    poly.add_vertex((-1.0583333, 7.672919999999976));
    poly.add_vertex((-3.175, 8.466670000000022));
    poly.add_vertex((-3.4395833, 9.26042000000001));
    poly.add_vertex((-3.7041665999999998, 8.466670000000022));
    poly.add_vertex((-5.8208333, 7.672920000000033));
    poly.add_vertex((-5.0270833, 4.233330000000024));
    poly.add_vertex((-4.2333333, 3.96875));
    poly.add_vertex((-4.2333333, 3.439579999999978));
    poly.add_vertex((-3.96875, 3.439579999999978));
    poly.add_vertex((-3.96875, 2.910419999999988));
    poly.add_vertex((-2.6458333, 2.6458299999999895));
    poly.add_vertex((-1.8520834000000002, 2.381249999999966));
    poly.add_vertex((-1.0583333000000001, 1.3229199999999537));
    poly.add_vertex((-0.7937500200000002, 0.5291699999999651));
    poly.add_vertex((-0.7937500200000002, -0.0));

    poly.add_index(44); poly.add_index(0); poly.add_index(1);
    poly.add_index(4); poly.add_index(5); poly.add_index(6);
    poly.add_index(6); poly.add_index(7); poly.add_index(8);
    poly.add_index(8); poly.add_index(9); poly.add_index(10);
    poly.add_index(8); poly.add_index(10); poly.add_index(11);
    poly.add_index(11); poly.add_index(12); poly.add_index(13);
    poly.add_index(13); poly.add_index(14); poly.add_index(15);
    poly.add_index(13); poly.add_index(15); poly.add_index(16);
    poly.add_index(16); poly.add_index(17); poly.add_index(18);
    poly.add_index(19); poly.add_index(20); poly.add_index(21);
    poly.add_index(19); poly.add_index(21); poly.add_index(22);
    poly.add_index(22); poly.add_index(23); poly.add_index(24);
    poly.add_index(22); poly.add_index(24); poly.add_index(25);
    poly.add_index(26); poly.add_index(27); poly.add_index(28);
    poly.add_index(28); poly.add_index(29); poly.add_index(30);
    poly.add_index(28); poly.add_index(30); poly.add_index(31);
    poly.add_index(31); poly.add_index(32); poly.add_index(33);
    poly.add_index(33); poly.add_index(34); poly.add_index(35);
    poly.add_index(33); poly.add_index(35); poly.add_index(36);
    poly.add_index(36); poly.add_index(37); poly.add_index(38);
    poly.add_index(38); poly.add_index(39); poly.add_index(40);
    poly.add_index(43); poly.add_index(44); poly.add_index(1);
    poly.add_index(43); poly.add_index(1); poly.add_index(2);
    poly.add_index(3); poly.add_index(4); poly.add_index(6);
    poly.add_index(3); poly.add_index(6); poly.add_index(8);
    poly.add_index(8); poly.add_index(11); poly.add_index(13);
    poly.add_index(8); poly.add_index(13); poly.add_index(16);
    poly.add_index(8); poly.add_index(16); poly.add_index(18);
    poly.add_index(26); poly.add_index(28); poly.add_index(31);
    poly.add_index(26); poly.add_index(31); poly.add_index(33);
    poly.add_index(26); poly.add_index(33); poly.add_index(36);
    poly.add_index(26); poly.add_index(36); poly.add_index(38);
    poly.add_index(38); poly.add_index(40); poly.add_index(41);
    poly.add_index(42); poly.add_index(43); poly.add_index(2);
    poly.add_index(8); poly.add_index(18); poly.add_index(19);
    poly.add_index(25); poly.add_index(26); poly.add_index(38);
    poly.add_index(3); poly.add_index(8); poly.add_index(19);
    poly.add_index(22); poly.add_index(25); poly.add_index(38);
    poly.add_index(22); poly.add_index(38); poly.add_index(41);
    poly.add_index(3); poly.add_index(19); poly.add_index(22);
    poly.add_index(22); poly.add_index(41); poly.add_index(42);
    poly.add_index(2); poly.add_index(3); poly.add_index(22);
    poly.add_index(22); poly.add_index(42); poly.add_index(2);
    asset.add_polygon(poly);

    assets.add_asset("lamp".to_string(), 
                     "2".to_string(), 
                     asset);
    
    


    let mut asset = asset::Asset::new();
    let mut poly = asset::AssetPolygon::new((0.2823529411764706,
                                             0.5294117647058824,
                                             0.0392156862745098) );
    poly.add_vertex((0.26458333, -0.0));
    poly.add_vertex((0.26458333, 2.6458299999999895));
    poly.add_vertex((6.3500002, 2.6458299999999895));
    poly.add_vertex((6.3500002, 2.116669999999999));
    poly.add_vertex((2.6458333, -0.5291700000000219));
    poly.add_vertex((10.583333, -0.5291700000000219));
    poly.add_vertex((6.6145835, 2.116669999999999));
    poly.add_vertex((6.6145835, 2.910419999999988));
    poly.add_vertex((-0.26458333, 2.910419999999988));
    poly.add_vertex((-0.26458333, -0.0));

    poly.add_index(9); poly.add_index(0); poly.add_index(1);
    poly.add_index(3); poly.add_index(4); poly.add_index(5);
    poly.add_index(3); poly.add_index(5); poly.add_index(6);
    poly.add_index(8); poly.add_index(9); poly.add_index(1);
    poly.add_index(2); poly.add_index(3); poly.add_index(6);
    poly.add_index(2); poly.add_index(6); poly.add_index(7);
    poly.add_index(7); poly.add_index(8); poly.add_index(1);
    poly.add_index(7); poly.add_index(1); poly.add_index(2);
    asset.add_polygon(poly);

    assets.add_asset("lamp".to_string(), 
                     "3".to_string(), 
                     asset);
    
    


    let mut asset = asset::Asset::new();
    let mut poly = asset::AssetPolygon::new((0.2823529411764706,
                                             0.5294117647058824,
                                             0.0392156862745098) );
    poly.add_vertex((0.79375002, -0.0));
    poly.add_vertex((0.79375002, 0.5291700000000219));
    poly.add_vertex((1.5875, 0.7937499999999886));
    poly.add_vertex((2.38125, 4.233330000000024));
    poly.add_vertex((0.26458333, 5.027080000000012));
    poly.add_vertex((-0.0, 5.820830000000001));
    poly.add_vertex((-0.26458333, 5.027080000000012));
    poly.add_vertex((-2.38125, 4.233330000000024));
    poly.add_vertex((-1.5875, 0.7937499999999886));
    poly.add_vertex((-0.79375002, 0.5291700000000219));
    poly.add_vertex((-0.79375002, -0.0));

    poly.add_index(10); poly.add_index(0); poly.add_index(1);
    poly.add_index(1); poly.add_index(2); poly.add_index(3);
    poly.add_index(1); poly.add_index(3); poly.add_index(4);
    poly.add_index(4); poly.add_index(5); poly.add_index(6);
    poly.add_index(6); poly.add_index(7); poly.add_index(8);
    poly.add_index(6); poly.add_index(8); poly.add_index(9);
    poly.add_index(9); poly.add_index(10); poly.add_index(1);
    poly.add_index(9); poly.add_index(1); poly.add_index(4);
    poly.add_index(9); poly.add_index(4); poly.add_index(6);
    asset.add_polygon(poly);

    assets.add_asset("lamp".to_string(), 
                     "1".to_string(), 
                     asset);
    
    


    let mut asset = asset::Asset::new();
    let mut poly = asset::AssetPolygon::new((0.2823529411764706,
                                             0.5294117647058824,
                                             0.0392156862745098) );
    poly.add_vertex((2.1166667, -0.0));
    poly.add_vertex((2.1166667, 0.5291700000000219));
    poly.add_vertex((1.3229167000000002, 0.7937500000000455));
    poly.add_vertex((0.7937496400000001, 8.995830000000069));
    poly.add_vertex((0.5291666400000001, 8.995830000000069));
    poly.add_vertex((0.5291666400000001, 31.75));
    poly.add_vertex((-0.5291663599999998, 31.75));
    poly.add_vertex((-0.5291663599999998, 8.995830000000012));
    poly.add_vertex((-0.7937503599999998, 8.995830000000012));
    poly.add_vertex((-1.3229162999999997, 0.7937499999999886));
    poly.add_vertex((-2.1166662999999994, 0.5291699999999651));

    poly.add_index(10); poly.add_index(0); poly.add_index(1);
    poly.add_index(10); poly.add_index(1); poly.add_index(2);
    poly.add_index(2); poly.add_index(3); poly.add_index(4);
    poly.add_index(4); poly.add_index(5); poly.add_index(6);
    poly.add_index(4); poly.add_index(6); poly.add_index(7);
    poly.add_index(7); poly.add_index(8); poly.add_index(9);
    poly.add_index(9); poly.add_index(10); poly.add_index(2);
    poly.add_index(2); poly.add_index(4); poly.add_index(7);
    poly.add_index(2); poly.add_index(7); poly.add_index(9);
    asset.add_polygon(poly);

    assets.add_asset("lamppost".to_string(), 
                     "1".to_string(), 
                     asset);
    
    


    return assets;
}
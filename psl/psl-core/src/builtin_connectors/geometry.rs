use std::{fmt, str::FromStr};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct GeometryParams {
    pub type_: GeometryType,
    pub srid: i32,
}

#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub enum GeometryType {
    #[default]
    Geometry = 0,
    Point = 1,
    LineString = 2,
    Polygon = 3,
    MultiPoint = 4,
    MultiLineString = 5,
    MultiPolygon = 6,
    GeometryCollection = 7,
    CircularString = 8,
    CompoundCurve = 9,
    CurvePolygon = 10,
    MultiCurve = 11,
    MultiSurface = 12,
    // Curve = 13,
    // Surface = 14,,
    PolyhedralSurface = 15,
    Tin = 16,
    Triangle = 17,
    GeometryZ = 1000,
    PointZ = 1001,
    LineStringZ = 1002,
    PolygonZ = 1003,
    MultiPointZ = 1004,
    MultiLineStringZ = 1005,
    MultiPolygonZ = 1006,
    GeometryCollectionZ = 1007,
    CircularStringZ = 1008,
    CompoundCurveZ = 1009,
    CurvePolygonZ = 1010,
    MultiCurveZ = 1011,
    MultiSurfaceZ = 1012,
    // CurveZ = 1013,
    // SurfaceZ = 1014,
    PolyhedralSurfaceZ = 1015,
    TinZ = 1016,
    TriangleZ = 1017,
    GeometryM = 2000,
    PointM = 2001,
    LineStringM = 2002,
    PolygonM = 2003,
    MultiPointM = 2004,
    MultiLineStringM = 2005,
    MultiPolygonM = 2006,
    GeometryCollectionM = 2007,
    CircularStringM = 2008,
    CompoundCurveM = 2009,
    CurvePolygonM = 2010,
    MultiCurveM = 2011,
    MultiSurfaceM = 2012,
    // CurveM = 2013,
    // SurfaceM = 2014,
    PolyhedralSurfaceM = 2015,
    TinM = 2016,
    TriangleM = 2017,
    GeometryZM = 3000,
    PointZM = 3001,
    LineStringZM = 3002,
    PolygonZM = 3003,
    MultiPointZM = 3004,
    MultiLineStringZM = 3005,
    MultiPolygonZM = 3006,
    GeometryCollectionZM = 3007,
    CircularStringZM = 3008,
    CompoundCurveZM = 3009,
    CurvePolygonZM = 3010,
    MultiCurveZM = 3011,
    MultiSurfaceZM = 3012,
    // CurveZM = 3013,
    // SurfaceZM = 3014,
    PolyhedralSurfaceZM = 3015,
    TinZM = 3016,
    TriangleZM = 3017,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub enum GeometryDimension {
    #[default]
    XY,
    XYZ,
    XYM,
    XYZM,
}

impl fmt::Display for GeometryDimension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::XY => write!(f, "XY"),
            Self::XYZ => write!(f, "XYZ"),
            Self::XYM => write!(f, "XYM"),
            Self::XYZM => write!(f, "XYZM"),
        }
    }
}

impl GeometryType {
    pub fn is_extended(&self) -> bool {
        self.as_2d() > Self::GeometryCollection
    }

    pub fn is_geojson_compatible(&self) -> bool {
        !self.is_extended() && self.dimension() <= &GeometryDimension::XYZ
    }

    pub fn as_2d(&self) -> Self {
        Self::try_from(*self as u32 % 1000).unwrap()
    }

    pub fn dimension(&self) -> &'static GeometryDimension {
        match *self as u32 / 1000 {
            0 => &GeometryDimension::XY,
            1 => &GeometryDimension::XYZ,
            2 => &GeometryDimension::XYM,
            3 => &GeometryDimension::XYZM,
            _ => unreachable!(),
        }
    }
}

impl TryFrom<u32> for GeometryType {
    type Error = String;

    fn try_from(value: u32) -> Result<Self, String> {
        match value {
            0 => Ok(Self::Geometry),
            1 => Ok(Self::Point),
            2 => Ok(Self::LineString),
            3 => Ok(Self::Polygon),
            4 => Ok(Self::MultiPoint),
            5 => Ok(Self::MultiLineString),
            6 => Ok(Self::MultiPolygon),
            7 => Ok(Self::GeometryCollection),
            8 => Ok(Self::CircularString),
            9 => Ok(Self::CompoundCurve),
            10 => Ok(Self::CurvePolygon),
            11 => Ok(Self::MultiCurve),
            12 => Ok(Self::MultiSurface),
            // 13 => Ok(Self::Curve),
            // 14 => Ok(Self::Surface),
            15 => Ok(Self::PolyhedralSurface),
            16 => Ok(Self::Tin),
            17 => Ok(Self::Triangle),
            1000 => Ok(Self::GeometryZ),
            1001 => Ok(Self::PointZ),
            1002 => Ok(Self::LineStringZ),
            1003 => Ok(Self::PolygonZ),
            1004 => Ok(Self::MultiPointZ),
            1005 => Ok(Self::MultiLineStringZ),
            1006 => Ok(Self::MultiPolygonZ),
            1007 => Ok(Self::GeometryCollectionZ),
            1008 => Ok(Self::CircularStringZ),
            1009 => Ok(Self::CompoundCurveZ),
            1010 => Ok(Self::CurvePolygonZ),
            1011 => Ok(Self::MultiCurveZ),
            1012 => Ok(Self::MultiSurfaceZ),
            // 1013 => Ok(Self::CurveZ),
            // 1014 => Ok(Self::SurfaceZ),
            1015 => Ok(Self::PolyhedralSurfaceZ),
            1016 => Ok(Self::TinZ),
            1017 => Ok(Self::TriangleZ),
            2000 => Ok(Self::GeometryM),
            2001 => Ok(Self::PointM),
            2002 => Ok(Self::LineStringM),
            2003 => Ok(Self::PolygonM),
            2004 => Ok(Self::MultiPointM),
            2005 => Ok(Self::MultiLineStringM),
            2006 => Ok(Self::MultiPolygonM),
            2007 => Ok(Self::GeometryCollectionM),
            2008 => Ok(Self::CircularStringM),
            2009 => Ok(Self::CompoundCurveM),
            2010 => Ok(Self::CurvePolygonM),
            2011 => Ok(Self::MultiCurveM),
            2012 => Ok(Self::MultiSurfaceM),
            // 2013 => Ok(Self::CurveM),
            // 2014 => Ok(Self::SurfaceM),
            2015 => Ok(Self::PolyhedralSurfaceM),
            2016 => Ok(Self::TinM),
            2017 => Ok(Self::TriangleM),
            3000 => Ok(Self::GeometryZM),
            3001 => Ok(Self::PointZM),
            3002 => Ok(Self::LineStringZM),
            3003 => Ok(Self::PolygonZM),
            3004 => Ok(Self::MultiPointZM),
            3005 => Ok(Self::MultiLineStringZM),
            3006 => Ok(Self::MultiPolygonZM),
            3007 => Ok(Self::GeometryCollectionZM),
            3008 => Ok(Self::CircularStringZM),
            3009 => Ok(Self::CompoundCurveZM),
            3010 => Ok(Self::CurvePolygonZM),
            3011 => Ok(Self::MultiCurveZM),
            3012 => Ok(Self::MultiSurfaceZM),
            // 3013 => Ok(Self::CurveZM),
            // 3014 => Ok(Self::SurfaceZM),
            3015 => Ok(Self::PolyhedralSurfaceZM),
            3016 => Ok(Self::TinZM),
            3017 => Ok(Self::TriangleZM),
            i => Err(format!("Invalid geometry type code: {i}")),
        }
    }
}

impl FromStr for GeometryType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "geometry" => Ok(GeometryType::Geometry),
            "geometrym" => Ok(GeometryType::GeometryM),
            "geometryz" => Ok(GeometryType::GeometryZ),
            "geometryzm" => Ok(GeometryType::GeometryZM),
            "point" => Ok(GeometryType::Point),
            "pointm" => Ok(GeometryType::PointM),
            "pointz" => Ok(GeometryType::PointZ),
            "pointzm" => Ok(GeometryType::PointZM),
            "linestring" => Ok(GeometryType::LineString),
            "linestringm" => Ok(GeometryType::LineStringM),
            "linestringz" => Ok(GeometryType::LineStringZ),
            "linestringzm" => Ok(GeometryType::LineStringZM),
            "polygon" => Ok(GeometryType::Polygon),
            "polygonm" => Ok(GeometryType::PolygonM),
            "polygonz" => Ok(GeometryType::PolygonZ),
            "polygonzm" => Ok(GeometryType::PolygonZM),
            "multipoint" => Ok(GeometryType::MultiPoint),
            "multipointm" => Ok(GeometryType::MultiPointM),
            "multipointz" => Ok(GeometryType::MultiPointZ),
            "multipointzm" => Ok(GeometryType::MultiPointZM),
            "multilinestring" => Ok(GeometryType::MultiLineString),
            "multilinestringm" => Ok(GeometryType::MultiLineStringM),
            "multilinestringz" => Ok(GeometryType::MultiLineStringZ),
            "multilinestringzm" => Ok(GeometryType::MultiLineStringZM),
            "multipolygon" => Ok(GeometryType::MultiPolygon),
            "multipolygonm" => Ok(GeometryType::MultiPolygonM),
            "multipolygonz" => Ok(GeometryType::MultiPolygonZ),
            "multipolygonzm" => Ok(GeometryType::MultiPolygonZM),
            "geometrycollection" => Ok(GeometryType::GeometryCollection),
            "geometrycollectionm" => Ok(GeometryType::GeometryCollectionM),
            "geometrycollectionz" => Ok(GeometryType::GeometryCollectionZ),
            "geometrycollectionzm" => Ok(GeometryType::GeometryCollectionZM),
            "circularstring" => Ok(GeometryType::CircularString),
            "circularstringm" => Ok(GeometryType::CircularStringM),
            "circularstringz" => Ok(GeometryType::CircularStringZ),
            "circularstringzm" => Ok(GeometryType::CircularStringZM),
            "compoundcurve" => Ok(GeometryType::CompoundCurve),
            "compoundcurvem" => Ok(GeometryType::CompoundCurveM),
            "compoundcurvez" => Ok(GeometryType::CompoundCurveZ),
            "compoundcurvezm" => Ok(GeometryType::CompoundCurveZM),
            "curvepolygon" => Ok(GeometryType::CurvePolygon),
            "curvepolygonm" => Ok(GeometryType::CurvePolygonM),
            "curvepolygonz" => Ok(GeometryType::CurvePolygonZ),
            "curvepolygonzm" => Ok(GeometryType::CurvePolygonZM),
            "multicurve" => Ok(GeometryType::MultiCurve),
            "multicurvem" => Ok(GeometryType::MultiCurveM),
            "multicurvez" => Ok(GeometryType::MultiCurveZ),
            "multicurvezm" => Ok(GeometryType::MultiCurveZM),
            "multisurface" => Ok(GeometryType::MultiSurface),
            "multisurfacem" => Ok(GeometryType::MultiSurfaceM),
            "multisurfacez" => Ok(GeometryType::MultiSurfaceZ),
            "multisurfacezm" => Ok(GeometryType::MultiSurfaceZM),
            "polyhedralsurface" => Ok(GeometryType::PolyhedralSurface),
            "polyhedralsurfacem" => Ok(GeometryType::PolyhedralSurfaceM),
            "polyhedralsurfacez" => Ok(GeometryType::PolyhedralSurfaceZ),
            "polyhedralsurfacezm" => Ok(GeometryType::PolyhedralSurfaceZM),
            "triangle" => Ok(GeometryType::Triangle),
            "trianglem" => Ok(GeometryType::TriangleM),
            "trianglez" => Ok(GeometryType::TriangleZ),
            "trianglezm" => Ok(GeometryType::TriangleZM),
            "tin" => Ok(GeometryType::Tin),
            "tinm" => Ok(GeometryType::TinM),
            "tinz" => Ok(GeometryType::TinZ),
            "tinzm" => Ok(GeometryType::TinZM),
            _ => Err(format!("{} is not a valid geometry type.", s)),
        }
    }
}

impl fmt::Display for GeometryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GeometryType::Geometry => write!(f, "Geometry"),
            GeometryType::GeometryM => write!(f, "GeometryM"),
            GeometryType::GeometryZ => write!(f, "GeometryZ"),
            GeometryType::GeometryZM => write!(f, "GeometryZM"),
            GeometryType::Point => write!(f, "Point"),
            GeometryType::PointM => write!(f, "PointM"),
            GeometryType::PointZ => write!(f, "PointZ"),
            GeometryType::PointZM => write!(f, "PointZM"),
            GeometryType::LineString => write!(f, "LineString"),
            GeometryType::LineStringM => write!(f, "LineStringM"),
            GeometryType::LineStringZ => write!(f, "LineStringZ"),
            GeometryType::LineStringZM => write!(f, "LineStringZM"),
            GeometryType::Polygon => write!(f, "Polygon"),
            GeometryType::PolygonM => write!(f, "PolygonM"),
            GeometryType::PolygonZ => write!(f, "PolygonZ"),
            GeometryType::PolygonZM => write!(f, "PolygonZM"),
            GeometryType::MultiPoint => write!(f, "MultiPoint"),
            GeometryType::MultiPointM => write!(f, "MultiPointM"),
            GeometryType::MultiPointZ => write!(f, "MultiPointZ"),
            GeometryType::MultiPointZM => write!(f, "MultiPointZM"),
            GeometryType::MultiLineString => write!(f, "MultiLineString"),
            GeometryType::MultiLineStringM => write!(f, "MultiLineStringM"),
            GeometryType::MultiLineStringZ => write!(f, "MultiLineStringZ"),
            GeometryType::MultiLineStringZM => write!(f, "MultiLineStringZM"),
            GeometryType::MultiPolygon => write!(f, "MultiPolygon"),
            GeometryType::MultiPolygonM => write!(f, "MultiPolygonM"),
            GeometryType::MultiPolygonZ => write!(f, "MultiPolygonZ"),
            GeometryType::MultiPolygonZM => write!(f, "MultiPolygonZM"),
            GeometryType::GeometryCollection => write!(f, "GeometryCollection"),
            GeometryType::GeometryCollectionM => write!(f, "GeometryCollectionM"),
            GeometryType::GeometryCollectionZ => write!(f, "GeometryCollectionZ"),
            GeometryType::GeometryCollectionZM => write!(f, "GeometryCollectionZM"),
            GeometryType::CircularString => write!(f, "CircularString"),
            GeometryType::CircularStringM => write!(f, "CircularStringM"),
            GeometryType::CircularStringZ => write!(f, "CircularStringZ"),
            GeometryType::CircularStringZM => write!(f, "CircularStringZM"),
            GeometryType::CompoundCurve => write!(f, "CompoundCurve"),
            GeometryType::CompoundCurveM => write!(f, "CompoundCurveM"),
            GeometryType::CompoundCurveZ => write!(f, "CompoundCurveZ"),
            GeometryType::CompoundCurveZM => write!(f, "CompoundCurveZM"),
            GeometryType::CurvePolygon => write!(f, "CurvePolygon"),
            GeometryType::CurvePolygonM => write!(f, "CurvePolygonM"),
            GeometryType::CurvePolygonZ => write!(f, "CurvePolygonZ"),
            GeometryType::CurvePolygonZM => write!(f, "CurvePolygonZM"),
            GeometryType::MultiCurve => write!(f, "MultiCurve"),
            GeometryType::MultiCurveM => write!(f, "MultiCurveM"),
            GeometryType::MultiCurveZ => write!(f, "MultiCurveZ"),
            GeometryType::MultiCurveZM => write!(f, "MultiCurveZM"),
            GeometryType::MultiSurface => write!(f, "MultiSurface"),
            GeometryType::MultiSurfaceM => write!(f, "MultiSurfaceM"),
            GeometryType::MultiSurfaceZ => write!(f, "MultiSurfaceZ"),
            GeometryType::MultiSurfaceZM => write!(f, "MultiSurfaceZM"),
            GeometryType::PolyhedralSurface => write!(f, "PolyhedralSurface"),
            GeometryType::PolyhedralSurfaceM => write!(f, "PolyhedralSurfaceM"),
            GeometryType::PolyhedralSurfaceZ => write!(f, "PolyhedralSurfaceZ"),
            GeometryType::PolyhedralSurfaceZM => write!(f, "PolyhedralSurfaceZM"),
            GeometryType::Triangle => write!(f, "Triangle"),
            GeometryType::TriangleM => write!(f, "TriangleM"),
            GeometryType::TriangleZ => write!(f, "TriangleZ"),
            GeometryType::TriangleZM => write!(f, "TriangleZM"),
            GeometryType::Tin => write!(f, "Tin"),
            GeometryType::TinM => write!(f, "TinM"),
            GeometryType::TinZ => write!(f, "TinZ"),
            GeometryType::TinZM => write!(f, "TinZM"),
        }
    }
}

impl crate::datamodel_connector::NativeTypeArguments for GeometryParams {
    const DESCRIPTION: &'static str = "a geometry type and an optional srid";
    const OPTIONAL_ARGUMENTS_COUNT: usize = 0;
    const REQUIRED_ARGUMENTS_COUNT: usize = 2;

    fn from_parts(parts: &[String]) -> Option<Self> {
        match parts {
            [geom] => GeometryType::from_str(geom).ok().map(|ty| Self { type_: ty, srid: 0 }),
            [geom, srid] => GeometryType::from_str(geom)
                .ok()
                .and_then(|ty| srid.parse().ok().map(|srid| Self { type_: ty, srid })),
            _ => None,
        }
    }

    fn to_parts(&self) -> Vec<String> {
        match self.srid {
            0 => vec![self.type_.to_string()],
            srid => vec![self.type_.to_string(), srid.to_string()],
        }
    }
}

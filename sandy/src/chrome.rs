use crate::*;
use bevy::pbr::StandardMaterial;
use mlua::LuaSerdeExt;
use ztransform::ZTransform;

#[derive(Debug, Component)]
pub struct Chrome {
    pub on_tick: mlua::Function,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChromePart {
    pub mesh: ChromeMesh,
    pub material: ChromeMaterial,
    pub offset: ZTransform,
}

impl mlua::FromLua for ChromePart {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        let table = value
            .as_table()
            .ok_or(mlua::Error::FromLuaConversionError {
                from: "value",
                to: "ChromPart".to_string(),
                message: None,
            })?;

        let mesh = lua
            .from_value(table.get("mesh").unwrap_or(mlua::Value::Nil))
            .unwrap_or_default();
        let material = lua
            .from_value(table.get("material").unwrap_or(mlua::Value::Nil))
            .unwrap_or_default();
        let offset = table.get("offset").unwrap_or(ZTransform::default());

        Ok(ChromePart {
            mesh,
            material,
            offset,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChromeMesh {
    Cylinder {
        radius: f32,
        height: f32,
    },
    Cuboid {
        x: f32,
        y: f32,
        z: f32,
    },
    CapsulePrism {
        radius: f32,
        length: f32,
        depth: f32,
    },
    Sphere {
        radius: f32,
    },
}

impl Default for ChromeMesh {
    fn default() -> Self {
        ChromeMesh::Sphere { radius: 1.0 }
    }
}

impl ChromeMesh {
    pub fn as_mesh(&self) -> Mesh {
        match self {
            ChromeMesh::Cylinder { radius, height } => {
                Cylinder::new(*radius, *height).mesh().build()
            }
            ChromeMesh::Cuboid { x, y, z } => Cuboid::new(*x, *y, *z).mesh().build(),
            ChromeMesh::CapsulePrism {
                radius,
                length,
                depth,
            } => Extrusion::new(Capsule2d::new(*radius, *length), *depth)
                .mesh()
                .build(),
            ChromeMesh::Sphere { radius } => Sphere::new(*radius).mesh().build(),
        }
    }

    pub fn lua_sphere(lua: &mlua::Lua, value: mlua::Value) -> Result<mlua::Value, mlua::Error> {
        let mesh = match value {
            mlua::Value::Number(f) => ChromeMesh::Sphere { radius: f as f32 },
            _ => ChromeMesh::Sphere { radius: 1.0 },
        };
        lua.to_value(&mesh)
    }
    pub fn lua_cuboid(lua: &mlua::Lua, value: mlua::Value) -> Result<mlua::Value, mlua::Error> {
        let mesh = match value {
            mlua::Value::Number(f) => {
                let f = f as f32;
                ChromeMesh::Cuboid { x: f, y: f, z: f }
            }
            mlua::Value::Table(table) => {
                let x_length = table.get("x").unwrap_or(1.0);
                let y_length = table.get("y").unwrap_or(1.0);
                let z_length = table.get("z").unwrap_or(1.0);
                ChromeMesh::Cuboid {
                    x: x_length,
                    y: y_length,
                    z: z_length,
                }
            }
            _ => ChromeMesh::Cuboid {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
        };
        lua.to_value(&mesh)
    }
    pub fn lua_cylinder(lua: &mlua::Lua, value: mlua::Value) -> Result<mlua::Value, mlua::Error> {
        let mesh = match value {
            mlua::Value::Number(f) => {
                let f = f as f32;
                ChromeMesh::Cylinder {
                    radius: f,
                    height: f,
                }
            }
            mlua::Value::Table(table) => {
                let radius = table.get("radius").unwrap_or(1.0);
                let height = table.get("height").unwrap_or(1.0);
                ChromeMesh::Cylinder { radius, height }
            }
            _ => ChromeMesh::Cylinder {
                radius: 1.0,
                height: 1.0,
            },
        };
        lua.to_value(&mesh)
    }
    pub fn lua_capsule_prism(
        lua: &mlua::Lua,
        value: mlua::Value,
    ) -> Result<mlua::Value, mlua::Error> {
        let mesh = match value {
            mlua::Value::Number(f) => {
                let f = f as f32;
                ChromeMesh::CapsulePrism {
                    radius: f,
                    length: f,
                    depth: f,
                }
            }
            mlua::Value::Table(table) => {
                let radius = table.get("radius").unwrap_or(1.0);
                let length = table.get("length").unwrap_or(1.0);
                let depth = table.get("depth").unwrap_or(1.0);
                ChromeMesh::CapsulePrism {
                    radius,
                    length,
                    depth,
                }
            }
            _ => ChromeMesh::CapsulePrism {
                radius: 1.0,
                length: 1.0,
                depth: 1.0,
            },
        };
        lua.to_value(&mesh)
    }
}

pub struct MeshChip;

impl lua::LuaChip for MeshChip {
    fn build(&self, lua: &mut lua::SandyLua) {
        let mesh = lua.create_table().unwrap();

        let sphere = lua.create_function(ChromeMesh::lua_sphere).unwrap();
        mesh.set("sphere", sphere).unwrap();

        let cuboid = lua.create_function(ChromeMesh::lua_cuboid).unwrap();
        mesh.set("cuboid", cuboid).unwrap();

        let cylinder = lua.create_function(ChromeMesh::lua_cylinder).unwrap();
        mesh.set("cylinder", cylinder).unwrap();

        let capsule_prism = lua.create_function(ChromeMesh::lua_capsule_prism).unwrap();
        mesh.set("capsule_prism", capsule_prism).unwrap();

        lua.globals().set("Mesh", mesh).unwrap();
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ChromeMaterial {
    pub color: ChromeColor,
    pub emissive: ChromeColor,
    pub metallic: f32,
    pub clearcoat: f32,
}

impl ChromeMaterial {
    pub fn as_material(&self) -> StandardMaterial {
        StandardMaterial {
            base_color: self.color.0,
            emissive: self.emissive.0.into(),
            metallic: self.metallic,
            clearcoat: self.clearcoat,
            ..Default::default()
        }
    }
}

impl Default for ChromeMaterial {
    fn default() -> Self {
        ChromeMaterial {
            color: ChromeColor::WHITE,
            emissive: ChromeColor::BLACK,
            metallic: 0.0,
            clearcoat: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChromeColor(Color);

impl ChromeColor {
    const WHITE: ChromeColor = ChromeColor(Color::WHITE);
    const BLACK: ChromeColor = ChromeColor(Color::BLACK);

    pub fn lua_rgba(lua: &mlua::Lua, value: mlua::Value) -> Result<mlua::Value, mlua::Error> {
        let c = match value {
            mlua::Value::Boolean(b) => {
                if b {
                    ChromeColor::WHITE
                } else {
                    ChromeColor::BLACK
                }
            }
            mlua::Value::Number(f) => ChromeColor(Color::linear_rgba(1.0, 1.0, 1.0, f as f32)),
            mlua::Value::Table(t) => {
                let red = t.get("red").unwrap_or_default();
                let green = t.get("green").unwrap_or_default();
                let blue = t.get("blue").unwrap_or_default();
                let alpha = t.get("alpha").unwrap_or_default();
                ChromeColor(Color::linear_rgba(red, green, blue, alpha))
            }
            _ => ChromeColor::WHITE,
        };
        lua.to_value(&c)
    }
    pub fn lua_hsva(lua: &mlua::Lua, value: mlua::Value) -> Result<mlua::Value, mlua::Error> {
        let c = match value {
            mlua::Value::Boolean(b) => {
                if b {
                    ChromeColor::WHITE
                } else {
                    ChromeColor::BLACK
                }
            }
            mlua::Value::Number(f) => ChromeColor(Color::hsva(1.0, 1.0, 1.0, f as f32)),
            mlua::Value::Table(t) => {
                let hue = t.get("hue").unwrap_or_default();
                let saturation = t.get("saturation").unwrap_or_default();
                let value = t.get("value").unwrap_or_default();
                let alpha = t.get("alpha").unwrap_or_default();
                ChromeColor(Color::hsva(hue, saturation, value, alpha))
            }
            _ => ChromeColor::WHITE,
        };
        lua.to_value(&c)
    }
}

pub struct ColorChip;

impl lua::LuaChip for ColorChip {
    fn build(&self, lua: &mut lua::SandyLua) {
        let color = lua.create_table().unwrap();

        let rgba = lua.create_function(ChromeColor::lua_rgba).unwrap();
        color.set("rgba", rgba).unwrap();

        let hsva = lua.create_function(ChromeColor::lua_hsva).unwrap();
        color.set("hsva", hsva).unwrap();

        lua.globals().set("Color", color).unwrap();
    }
}

//#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(default)]
//pub struct ChromeColor {
//    red: f32,
//    green: f32,
//    blue: f32,
//    alpha: f32,
//}
//
//impl Default for ChromeColor {
//    fn default() -> Self {
//        ChromeColor::WHITE.clone()
//    }
//}
//
//impl From<ChromeColor> for LinearRgba {
//    fn from(value: ChromeColor) -> Self {
//        LinearRgba::new(value.red, value.green, value.blue, value.alpha)
//    }
//}
//impl From<ChromeColor> for Color {
//    fn from(value: ChromeColor) -> Self {
//        let l : LinearRgba = value.into();
//        l.into()
//    }
//}
//impl From<Color> for ChromeColor {
//    fn from(value: Color) -> Self {
//        let l : LinearRgba = value.into();
//        ChromeColor { red: l.red, green: l.green, blue: l.blue, alpha: l.alpha }
//    }
//}
//
//impl ChromeColor {
//    const WHITE: ChromeColor = ChromeColor {
//        red: 1.0,
//        green: 1.0,
//        blue: 1.0,
//        alpha: 1.0,
//    };
//    const BLACK: ChromeColor = ChromeColor {
//        red: 0.0,
//        green: 0.0,
//        blue: 0.0,
//        alpha: 1.0,
//    };
//}

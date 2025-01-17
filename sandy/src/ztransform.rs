use bevy::prelude::*;
use mlua::FromLua;

use crate::*;

pub struct GeometryChip;
impl lua::LuaChip for GeometryChip {
    fn build(&self, lua: &mut lua::SandyLua) {
        let transform = lua.create_proxy::<ZTransform>().unwrap();
        lua.globals().set("Transform", transform).unwrap();
        let vector = lua.create_proxy::<ZVec3>().unwrap();
        lua.globals().set("Vector", vector).unwrap();
        let rotation = lua.create_proxy::<ZQuat>().unwrap();
        lua.globals().set("Rotation", rotation).unwrap();
    }
}

#[derive(Debug, Clone, Bundle, Default)]
pub struct ZBundle {
    pub transform: Transform,
    pub ztransform: ZTransform,
}

#[derive(Debug, Clone, Component, PartialEq, Default)]
pub struct ZTransform(pub Transform);

impl ZBundle {
    pub fn identity() -> ZBundle {
        ZBundle::new(Transform::IDENTITY)
    }

    pub fn new(ztransform: Transform) -> ZBundle {
        ZBundle {
            ztransform: ZTransform(ztransform),
            ..Default::default()
        }
    }

    pub fn from_xyz(x: f32, y: f32, z: f32) -> ZBundle {
        ZBundle::new(Transform::from_xyz(x, y, z))
    }
}

impl ZTransform {
    pub fn lua_from_xyz(_lua: &mlua::Lua, value: mlua::Value) -> Result<Self, mlua::Error> {
        let t = match value {
            mlua::Value::Table(table) => {
                let x = table.get("x").unwrap_or_default();
                let y = table.get("y").unwrap_or_default();
                let z = table.get("z").unwrap_or_default();
                Self(Transform::from_xyz(x, y, z))
            }
            _ => Self(Transform::IDENTITY),
        };
        Ok(t)
    }
    pub fn lua_from_vec_rot(
        lua: &mlua::Lua,
        (vec, rot): (mlua::Value, mlua::Value),
    ) -> Result<Self, mlua::Error> {
        let vec = ZVec3::from_lua(vec, lua).unwrap_or_default();
        let rot = ZQuat::from_lua(rot, lua).unwrap_or_default();

        Ok(ZTransform(
            Transform::from_translation(vec.0).with_rotation(rot.0),
        ))
    }
    fn lua_meta_mul(_lua: &mlua::Lua, this: &Self, other: Self) -> Result<Self, mlua::Error> {
        Ok(ZTransform(this.0 * other.0))
    }
}

impl mlua::UserData for ZTransform {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("from_xyz", ZTransform::lua_from_xyz);
        methods.add_function("from_vec_rot", ZTransform::lua_from_vec_rot);
        methods.add_meta_method(mlua::MetaMethod::Mul, ZTransform::lua_meta_mul);
    }
}

impl mlua::FromLua for ZTransform {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        match value {
            mlua::Value::UserData(any) => {
                let t: mlua::UserDataRef<ZTransform> = any.borrow()?;
                Ok(t.clone())
            }
            _ => Err(mlua::Error::FromLuaConversionError {
                from: "Transform",
                to: format!("{:?}", value),
                message: None,
            }),
        }
    }
}

#[derive(Debug, Clone, Component, PartialEq, Default)]
pub struct ZVec3(pub Vec3);

impl ZVec3 {
    pub fn lua_from_xyz(_lua: &mlua::Lua, value: mlua::Value) -> Result<Self, mlua::Error> {
        let t = match value {
            mlua::Value::Table(table) => {
                let x = table.get("x").unwrap_or_default();
                let y = table.get("y").unwrap_or_default();
                let z = table.get("z").unwrap_or_default();
                Self(Vec3::new(x, y, z))
            }
            mlua::Value::UserData(any) => {
                let t: mlua::UserDataRef<ZVec3> = any.borrow()?;
                t.clone()
            }
            _ => Self(Vec3::ZERO),
        };
        Ok(t)
    }
}

impl mlua::UserData for ZVec3 {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", ZVec3::lua_from_xyz)
    }
}

impl mlua::FromLua for ZVec3 {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        ZVec3::lua_from_xyz(lua, value)
    }
}

#[derive(Debug, Clone, Component, PartialEq, Default)]
pub struct ZQuat(pub Quat);

impl ZQuat {
    pub fn lua_from_rx(_lua: &mlua::Lua, angle: f32) -> Result<Self, mlua::Error> {
        Ok(ZQuat(Quat::from_rotation_x(angle.to_radians())))
    }
    pub fn lua_from_ry(_lua: &mlua::Lua, angle: f32) -> Result<Self, mlua::Error> {
        Ok(ZQuat(Quat::from_rotation_y(angle.to_radians())))
    }
    pub fn lua_from_rz(_lua: &mlua::Lua, angle: f32) -> Result<Self, mlua::Error> {
        Ok(ZQuat(Quat::from_rotation_z(angle.to_radians())))
    }
}

impl mlua::UserData for ZQuat {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("from_rx", ZQuat::lua_from_rx);
        methods.add_function("from_ry", ZQuat::lua_from_ry);
        methods.add_function("from_rz", ZQuat::lua_from_rz);
    }
}

impl mlua::FromLua for ZQuat {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        match value {
            mlua::Value::UserData(any) => {
                let t: mlua::UserDataRef<ZQuat> = any.borrow()?;
                Ok(t.clone())
            }
            _ => Err(mlua::Error::runtime(
                "Fail to convert lua value to Rotation",
            )),
        }
    }
}

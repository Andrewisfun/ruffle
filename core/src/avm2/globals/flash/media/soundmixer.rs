//! `flash.media.SoundMixer` builtin/prototype

use crate::avm2::activation::Activation;
use crate::avm2::class::{Class, ClassAttributes};
use crate::avm2::method::{Method, NativeMethodImpl};
use crate::avm2::names::{Namespace, QName};
use crate::avm2::object::{Object, TObject};
use crate::avm2::value::Value;
use crate::avm2::Error;
use crate::display_object::SoundTransform;
use gc_arena::{GcCell, MutationContext};

/// Implements `flash.media.SoundMixer`'s instance constructor.
pub fn instance_init<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    if let Some(this) = this {
        activation.super_init(this, &[])?;
    }

    Ok(Value::Undefined)
}

/// Implements `flash.media.SoundMixer`'s class constructor.
pub fn class_init<'gc>(
    _activation: &mut Activation<'_, 'gc, '_>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    Ok(Value::Undefined)
}

/// Implements `soundTransform`'s getter
///
/// This also implements `SimpleButton`'s `soundTransform` property, as per
/// Flash Player behavior.
pub fn sound_transform<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    let dobj_st = activation.context.global_sound_transform().clone();
    let mut as3_st = activation
        .avm2()
        .classes()
        .soundtransform
        .construct(activation, &[])?;

    as3_st.set_property(
        as3_st,
        &QName::new(Namespace::public(), "leftToLeft"),
        (dobj_st.left_to_left as f64 / 100.0).into(),
        activation,
    )?;
    as3_st.set_property(
        as3_st,
        &QName::new(Namespace::public(), "leftToRight"),
        (dobj_st.left_to_right as f64 / 100.0).into(),
        activation,
    )?;
    as3_st.set_property(
        as3_st,
        &QName::new(Namespace::public(), "rightToLeft"),
        (dobj_st.right_to_left as f64 / 100.0).into(),
        activation,
    )?;
    as3_st.set_property(
        as3_st,
        &QName::new(Namespace::public(), "rightToRight"),
        (dobj_st.right_to_right as f64 / 100.0).into(),
        activation,
    )?;
    as3_st.set_property(
        as3_st,
        &QName::new(Namespace::public(), "volume"),
        (dobj_st.volume as f64 / 100.0).into(),
        activation,
    )?;

    Ok(as3_st.into())
}

/// Implements `soundTransform`'s setter
///
/// This also implements `SimpleButton`'s `soundTransform` property, as per
/// Flash Player behavior.
pub fn set_sound_transform<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    _this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    let as3_st = args
        .get(0)
        .cloned()
        .unwrap_or(Value::Undefined)
        .coerce_to_object(activation)?;
    let dobj_st = SoundTransform {
        left_to_left: (as3_st
            .get_property(
                as3_st,
                &QName::new(Namespace::public(), "leftToLeft"),
                activation,
            )?
            .coerce_to_number(activation)?
            * 100.0) as i32,
        left_to_right: (as3_st
            .get_property(
                as3_st,
                &QName::new(Namespace::public(), "leftToRight"),
                activation,
            )?
            .coerce_to_number(activation)?
            * 100.0) as i32,
        right_to_left: (as3_st
            .get_property(
                as3_st,
                &QName::new(Namespace::public(), "rightToLeft"),
                activation,
            )?
            .coerce_to_number(activation)?
            * 100.0) as i32,
        right_to_right: (as3_st
            .get_property(
                as3_st,
                &QName::new(Namespace::public(), "rightToRight"),
                activation,
            )?
            .coerce_to_number(activation)?
            * 100.0) as i32,
        volume: (as3_st
            .get_property(
                as3_st,
                &QName::new(Namespace::public(), "volume"),
                activation,
            )?
            .coerce_to_number(activation)?
            * 100.0) as i32,
    };

    activation.context.set_global_sound_transform(dobj_st);

    Ok(Value::Undefined)
}

/// Construct `SoundMixer`'s class.
pub fn create_class<'gc>(mc: MutationContext<'gc, '_>) -> GcCell<'gc, Class<'gc>> {
    let class = Class::new(
        QName::new(Namespace::package("flash.media"), "SoundMixer"),
        Some(QName::new(Namespace::public(), "Object").into()),
        Method::from_builtin(instance_init, "<SoundMixer instance initializer>", mc),
        Method::from_builtin(class_init, "<SoundMixer class initializer>", mc),
        mc,
    );

    let mut write = class.write(mc);

    write.set_attributes(ClassAttributes::SEALED | ClassAttributes::FINAL);

    const PUBLIC_CLASS_PROPERTIES: &[(&str, Option<NativeMethodImpl>, Option<NativeMethodImpl>)] =
        &[(
            "soundTransform",
            Some(sound_transform),
            Some(set_sound_transform),
        )];
    write.define_public_builtin_class_properties(mc, PUBLIC_CLASS_PROPERTIES);

    class
}

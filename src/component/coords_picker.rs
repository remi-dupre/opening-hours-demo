use opening_hours::localization::{Coordinates, Country};
use web_sys::wasm_bindgen::JsCast;
use web_sys::wasm_bindgen::closure::ScopedClosure;
use web_sys::{GeolocationPosition, HtmlInputElement, window};
use yew::prelude::*;

use crate::component::icon::Icon;
use crate::component::icon::Position;
use crate::eval::{CallbackUpdateCtx, EvalContext};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub coords: Coordinates,
    pub cb_update_ctx: CallbackUpdateCtx,
    pub country: Option<Country>,
    pub timezone: chrono_tz::Tz,
}

#[function_component]
pub fn CoordsPicker(props: &Props) -> Html {
    let on_geolocate_click = use_callback(
        props.cb_update_ctx.clone(),
        |_input: MouseEvent, cb_update_ctx| {
            let geolocation = window().unwrap().navigator().geolocation().unwrap();

            let coords_js_cb = ScopedClosure::wrap(Box::new(move |pos: GeolocationPosition| {
                let coords_js = pos.coords();

                let Some(coords) = Coordinates::new(coords_js.latitude(), coords_js.longitude())
                else {
                    log::error!(
                        "Invalid coordinates from JS: ({}, {})",
                        coords_js.latitude(),
                        coords_js.longitude()
                    );

                    return;
                };

                cb_update_ctx.emit(Box::new(move |ctx| ctx.with_coords(coords)));
            }) as Box<dyn FnMut(_)>);

            geolocation.get_current_position(coords_js_cb.as_ref().unchecked_ref());
            coords_js_cb.forget();
        },
    );

    let on_input_update = |setter: Box<dyn Fn(f64) -> _>| {
        move |input: InputEvent, cb_update_ctx: &CallbackUpdateCtx| {
            let Some(el) = input.target_dyn_into::<HtmlInputElement>() else {
                log::error!("Could not fetch latitude input");
                return;
            };

            let val = match el.value().parse() {
                Ok(val) => val,
                Err(err) => {
                    log::warn!("Invalid input latitude: {err}");
                    return;
                }
            };

            cb_update_ctx.emit(setter(val))
        }
    };

    let set_lat = Box::new(|lat| {
        Box::new(move |ctx: EvalContext| {
            let coords = ctx.get_coords();
            ctx.with_coords(Coordinates::new(lat, coords.lon()).unwrap_or(coords))
        }) as _
    });

    let set_lon = Box::new(|lon| {
        Box::new(move |ctx: EvalContext| {
            let coords = ctx.get_coords();
            ctx.with_coords(Coordinates::new(coords.lat(), lon).unwrap_or(coords))
        }) as _
    });

    let on_update_lat = use_callback(props.cb_update_ctx.clone(), on_input_update(set_lat));
    let on_update_lon = use_callback(props.cb_update_ctx.clone(), on_input_update(set_lon));

    html! {
      <>
        <fieldset role="group">
          <input
            type="number"
            step="0.01"
            placeholder="Latitude"
            value={props.coords.lat().to_string()}
            oninput={on_update_lat}
          />
          <input
            type="number"
            step="0.01"
            placeholder="Longitude"
            value={props.coords.lon().to_string()}
            oninput={on_update_lon}
          />
          <button onclick={on_geolocate_click}>
            <Icon src={Position} />
          </button>
        </fieldset>
        <small>
          {"Detected country: "}
          <i>{props.country.map(Country::name).unwrap_or("None")}</i>
          {"."}
          <br />
          {"Detected timezone: "}
          <i>{props.timezone.name()}</i>
          {"."}
        </small>
      </>
    }
}

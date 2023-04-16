import {
    Kind,
} from "./kind_generated.js"
import {
    fill_circle_2,
} from "../canvas.js"


const drawers = {
    [Kind.Booster]: (context, context_trace, diameter, p) => {
        if (p.a == 1) {
            fill_circle_2(context, p.pout2, diameter*0.7, "#c22")
            fill_circle_2(context, p.pout, diameter*0.9, "#c00")
            fill_circle_2(context, p.p, diameter*1, "#d20")
            fill_circle_2(context_trace, p.p, diameter*1, "#d20")
        } else {
            fill_circle_2(context, p.p, diameter*1, "#b40")
        }
    },
    [Kind.Ray]: (context, context_trace, diameter, p) => {
        const r = 255*0.0
        const g = 255*( p.volume/2500 )
        const b = 255*( 0.5 + p.volume/2500 )
        fill_circle_2(context, p.p, diameter*1, `rgb(${r},${g},${b})`)
    },
    [Kind.Sun]: (context, context_trace, diameter, p) => {
        fill_circle_2(context_trace, p.p, diameter*1, `#ca28`)
        fill_circle_2(context_trace, p.pout2, diameter*0.7, "#ca28")
        fill_circle_2(context_trace, p.pout, diameter*0.9, "#ca28")
    },
    [Kind.ElectroField]: (context, context_trace, diameter, p) => {
        fill_circle_2(context, p.p, diameter*1, "#88f")
    },
    [Kind.PlasmaElectroField]: (context, context_trace, diameter, p) => {
        fill_circle_2(context, p.p, diameter*1, "#88f")
        fill_circle_2(context, p.p, diameter*0.5, "#da4")
        fill_circle_2(context_trace, p.p, diameter*1.5, "#ca28")
    },
    [Kind.Core]: (context, context_trace, diameter, p) => {
        fill_circle_2(context, p.p, diameter*1, "#c83")
    },
    [Kind.Static]: (context, context_trace, diameter, p) => {
        fill_circle_2(context, p.p, diameter*1, "#da4")
    },
    [Kind.Armor]: (context, context_trace, diameter, p) => {
        fill_circle_2(context, p.p, diameter*1, "#da4")
    },
    [Kind.SunCore]: (context, context_trace, diameter, p) => {
        fill_circle_2(context, p.p, diameter*1, "#da4")
    },
    [Kind.Anchor]: (context, context_trace, diameter, p) => {
        fill_circle_2(context, p.p, diameter*1, "#da4")
    },
    [Kind.PlasmaDepot]: (context, context_trace, diameter, p) => {
        fill_circle_2(context, p.p, diameter*1, "#df4")
    },
    [Kind.PlasmaCollector]: (context, context_trace, diameter, p) => {
        fill_circle_2(context, p.p, diameter*1, "#da4")
    },
    [Kind.PlasmaCargo]: (context, context_trace, diameter, p) => {
        fill_circle_2(context, p.p, diameter*1, "#da4")
    },
    [Kind.Default]: (context, context_trace, diameter, p) => {
        fill_circle_2(context, p.p, diameter*1, "#f0f")
    },
}


const draw_particle = (context, context_trace, diameter, p) => {
    const drawer = drawers[p.k]
    if (drawer) {
        drawer(context, context_trace, diameter, p)
    } else {
        fill_circle_2(context, p.p, diameter*30, "#f0f")
        throw(p.k)
    }
}


export {
    draw_particle,
}

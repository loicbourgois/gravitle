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
        const g = 255*( p.quantity/2500 )
        const b = 255*( 0.5 + p.quantity/2500 )
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
        if (p.a == 1) {
            fill_circle_2(context, p.p, diameter*1, "#f64")
        } else {
            fill_circle_2(context, p.p, diameter*1, "#da4")
            const r = 0
            const g = 255
            const b = 200
            fill_circle_2(context, p.p, diameter*1, `rgb(${r},${g},${b})`)
        }
    },
    [Kind.PlasmaCargo]: (context, context_trace, diameter, p) => {
        fill_circle_2(context, p.p, diameter*1, "#da4")
    },
    [Kind.Default]: (context, context_trace, diameter, p) => {
        fill_circle_2(context, p.p, diameter*1, "#f0f")
    },
    [Kind.PlasmaRefineryInput]: (context, context_trace, diameter, p) => {
        fill_circle_2(context, p.p, diameter*1, "#f0f")
    },
    [Kind.PlasmaRefineryOutput]: (context, context_trace, diameter, p) => {
        fill_circle_2(context, p.p, diameter*1, "#f0f")
    },
    [Kind.CoalAsteroid]: (context, context_trace, diameter, p) => {
        fill_circle_2(context, p.p, diameter*1, "#444")
    },
    [Kind.CoalDepot]: (context, context_trace, diameter, p) => {
        fill_circle_2(context, p.p, diameter*1, "#444")
    },
    [Kind.IronAsteroid]: (context, context_trace, diameter, p) => {
        fill_circle_2(context, p.p, diameter*1, "#888")
    },
    [Kind.IronOreDepot]: (context, context_trace, diameter, p) => {
        fill_circle_2(context, p.p, diameter*1, "#888")
    },
    [Kind.Light]: (context, context_trace, diameter, p) => {
        fill_circle_2(context, p.p, diameter*1, "#FF4")
    },
    [Kind.EnergyCollector]: (context, context_trace, diameter, p) => {
        fill_circle_2(context, p.p, diameter*1, "#B80")
    },
    [Kind.CoalCargo]: (context, context_trace, diameter, p) => {
        fill_circle_2(context, p.p, diameter*1, "#B80")
    },
    [Kind.Battery]: (context, context_trace, diameter, p) => {
        fill_circle_2(context, p.p, diameter*1, "#B80")
    },
    [Kind.CoalCollector]: (context, context_trace, diameter, p) => {
        fill_circle_2(context, p.p, diameter*1, "#B80")
    },
    [Kind.IronOreCargo]: (context, context_trace, diameter, p) => {
        fill_circle_2(context, p.p, diameter*1, "#B80")
    },
    [Kind.IronOreCollector]: (context, context_trace, diameter, p) => {
        fill_circle_2(context, p.p, diameter*1, "#B80")
    },
    
}


const draw_particle = (context, context_trace, diameter, p) => {
    const drawer = drawers[p.k]
    if (drawer) {
        drawer(context, context_trace, diameter, p)
    } else {
        fill_circle_2(context, p.p, diameter*30, "#f0f")
        throw(`cannot draw: invalid kind: ${p.k}`)
    }
    // if (p.a) {
    //     fill_circle_2(context, p.p, diameter*3, "#f0f")
    // }
}


export {
    draw_particle,
}

diatom = {
    'name': 'Diatom',
    'description': 'Jewel of the microverse',
    'class': 'solid',
    'storage': {
        'plant': 0.09,
        'water': 0.5,
        'light': 0.01,
        'heat':  0.2
    },
    'transfer': {
        'plant': 0.0,
        'water': 0.001,
        'light': 0.001,
        'heat':  0.001
    },
    'formulas' : {
        {
            'light': -0.001,
            'plant':  0.001,
        }
    },
    'transforms': {
        'frozen_diatom': {
            'heat': 'min',
        },
        'burning_diatom': {
            'heat': 'max',
        }
    },
    'fission': {
        'conditions': {
            'plant': 'max',
            'water': 'max'
        },
    },
}

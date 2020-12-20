const conf = {
  red_amount: 0.8,
  interval_render_ms: 10,
  interval_controls_ms: 10,
  interval_main_ms: 10,
  render: {
    last_time: null
  },
  log_render_duration: false,
  'draw_inactive': false,
  'colors': {
    'energy': {
      'r': 1.0,
      'g': 1.0,
      'b': 0.0
    },
    'rock': {
      'r': 0.5,
      'g': 0.3,
      'b': 0.1
    },
    'matter': {
      'r': 0.0,
      'g': 0.9,
      'b': 0.2
    },
    'organic_matter': {
      'r': 0.0,
      'g': 0.9,
      'b': 0.9
    },
    'waste': {
      'r': 0.8,
      'g': 0.8,
      'b': 0.8
    },
    'metal': {
      'r': 1.0,
      'g': 1.0,
      'b': 0.7
    },
    'thruster': {
      'r': 1.0,
      'g': 0.7,
      'b': 0.0
    },
    'health': '#8fa',
    'body':   '#8cf',
    'egg':    '#faa',
    'body_up':'#8cf',
    'travel': '#a88',
    'link':   '#eee',
    'best_dna_ever_by_distance_traveled': '#faa',
    'best_dna_ever_by_age': '#afa',
    'best_dna_alive_by_age': '#aaf',
    'best_dna_alive_by_distance_traveled': '#ffa',
    'averages': '#aaa',
    'eye': {
      'white': '#eee',
      'black': '#111'
    },
    'mouth': {
      'back': {
        'r': 25.0,
        'g': 70.0,
        'b': 150.0
      },
      'top': {
        'r': 25.0,
        'g': 100.0,
        'b': 255.0
      }
    },
    'turbo': {
      'back': {
        'r': 192.0,
        'g': 0.0,
        'b': 0.0
      },
      'top': {
        'r': 255.0,
        'g': 255.0,
        'b': 0.0
      }
    },
    'line_of_sight': '#eea',
    'vision_points': '#eea'
  },
  'urls': [
    'ws://127.0.0.1:10001',
  ],
  'quotes': [
    `Welcome to VWorld`,
    `It's not a bug, it's a maladaptation to the current universe`,
    `One too many is too many`,
    `Value is at the edge`,
    `Focus`,
  ],
  'health_diameter_ratio': 0.5,
}

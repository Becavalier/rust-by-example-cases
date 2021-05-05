const rust = import('./pkg')
rust.then(m => m.run()).catch(console.error)

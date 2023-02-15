# "magik"-ify an image

Implements the popular "magik" image generation that Discord bots have.

# Examples

### Async version (recommended):

```js
import { Magik } from '@khaf/magik'
import { readFileSync } from 'fs'

const buffer = readFileSync('./image.png')
const magik = new Magik(buffer)

const image = await magik.magikify()
```

### Sync version

The time it takes to magikify an image depends on the image size. This might end blocking the thread for multiple minutes.

```js
import { magikSync } from './index.js'
import { readFileSync } from 'fs'

const buffer = readFileSync('./image.png')
const image = magikSync(buffer)
```

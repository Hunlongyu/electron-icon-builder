import {Command, Flags, CliUx} from '@oclif/core'
import path from 'node:path'
import fs from 'node:fs'
import icongen from 'icon-gen'
import photon from '@silvia-odwyer/photon-node'

export default class Run extends Command {
  static description = 'An icon generator to generate all the icon files needed for electron packaging'

  static examples = [
    '<%= config.bin %>',
    '<%= config.bin %> -i ./icon.png -o ./',
    '<%= config.bin %> -i ./icon.png -o ./ -f',
  ]

  static flags = {
    input: Flags.string({char: 'i', default: './icon.png'}),
    output: Flags.string({char: 'o', default: './build/'}),
    flatten: Flags.boolean({char: 'f', default: false}),
  }

  public async run(): Promise<void> {
    try {
      const {flags} = await this.parse(Run)
      const input = path.resolve(process.cwd(), flags.input)
      const output = path.resolve(process.cwd(), flags.output)
      const flatten = flags.flatten
      const win = flatten ? output : path.join(output, 'win')
      const mac = flatten ? output : path.join(output, 'mac')
      const png = flatten ? output : path.join(output, 'png')
      await this.ensureDirExists(output)
      if (!flatten) {
        await this.ensureDirExists(win)
        await this.ensureDirExists(mac)
        await this.ensureDirExists(png)
      }

      await this.next(input, win, mac, png)
    } catch (error) {
      console.log('=== error ===', error)
    }
  }

  private async ensureDirExists(dir: string) {
    if (!fs.existsSync(dir)) {
      fs.mkdirSync(dir)
    }
  }

  private async next(input: string, win: string, mac: string, png: string) {
    try {
      console.time('It takes: ')
      CliUx.ux.action.start('running')
      await icongen(input, win, {report: false, ico: {name: 'icon', sizes: [256]}})
      await icongen(input, mac, {report: false, icns: {name: 'icon', sizes: [512]}})
      await this.createPngs(input, png)
      CliUx.ux.action.stop('done!')
      console.timeEnd('It takes: ')
    } catch (error) {
      CliUx.ux.action.stop('error!')
      console.log('=== error ===', error)
    }
  }

  private async createPngs(input: string, png: string) {
    try {
      const base64 = fs.readFileSync(input, {encoding: 'base64'})
      const data = base64.replace(/^data:image\/(png|jpg);base64,/, '')
      const img = photon.PhotonImage.new_from_base64(data)
      const size = [16, 32, 48, 64, 128, 256, 512]
      for (const i of size) {
        const res = photon.resize(img, i, i, 5)
        const outputBase64 = res.get_base64()
        const outputName = `${png}/${i}x${i}.png`
        const outputData = outputBase64.replace(/^data:image\/\w+;base64,/, '')
        fs.writeFileSync(outputName, outputData, {encoding: 'base64'})
      }
    } catch (error) {
      console.log('=== error ===', error)
    }
  }
}

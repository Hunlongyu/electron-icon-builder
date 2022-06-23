import {Command, Flags} from '@oclif/core'

export default class Run extends Command {
  static description = 'An icon generator to generate all the icon files needed for electron packaging'

  static examples = [
    '<%= config.bin %>',
    '<%= config.bin %> -i ./icon.png -o ./',
    '<%= config.bin %> -i ./icon.png -o ./ -f',
  ]

  static flags = {
    input: Flags.string({char: 'i', default: './icon.png'}),
    output: Flags.string({char: 'o', default: './'}),
    flatten: Flags.boolean({char: 'f', default: false}),
  }

  public async run(): Promise<void> {
    const {flags} = await this.parse(Run)
    if (flags.input) console.log(`input: ${flags.input}`)
    if (flags.output) console.log(`output: ${flags.output}`)
    if (flags.flatten) console.log(`flatten: ${flags.flatten}`)
  }
}

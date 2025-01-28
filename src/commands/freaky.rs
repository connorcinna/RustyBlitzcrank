use crate::{Context, Error};

pub fn freaktionary(c: char) -> char
{
    match c
    {
        'a'=> '𝓪',
        'b'=> '𝓫',
        'c'=> '𝓬',
        'd'=> '𝓭',
        'e'=> '𝓮',
        'f'=> '𝓯',
        'g'=> '𝓰',
        'h'=> '𝓱',
        'i'=> '𝓲',
        'j'=> '𝓳',
        'k'=> '𝓴',
        'l'=> '𝓵',
        'm'=> '𝓶',
        'n'=> '𝓷',
        'o'=> '𝓸',
        'p'=> '𝓹',
        'q'=> '𝓺',
        'r'=> '𝓻',
        's'=> '𝓼',
        't'=> '𝓽',
        'u'=> '𝓾',
        'v'=> '𝓿',
        'w'=> '𝔀',
        'x'=> '𝔁',
        'y'=> '𝔂',
        'z'=> '𝔃',
        'A'=> '𝓐',
        'B'=> '𝓑',
        'C'=> '𝓒',
        'D'=> '𝓓',
        'E'=> '𝓔',
        'F'=> '𝓕',
        'G'=> '𝓖',
        'H'=> '𝓗',
        'I'=> '𝓘',
        'J'=> '𝓙',
        'K'=> '𝓚',
        'L'=> '𝓛',
        'M'=> '𝓜',
        'N'=> '𝓝',
        'O'=> '𝓞',
        'P'=> '𝓟',
        'Q'=> '𝓠',
        'R'=> '𝓡',
        'S'=> '𝓢',
        'T'=> '𝓣',
        'U'=> '𝓤',
        'V'=> '𝓥',
        'W'=> '𝓦',
        'X'=> '𝓧',
        'Y'=> '𝓨',
        'Z'=> '𝓩',
        ' ' => ' ',
        _ => c
    }
}

#[poise::command(slash_command)]
pub async fn run(
    ctx: Context<'_>,
    #[description = "get 𝓯𝓻𝓮𝓪𝓴𝔂 𝓿𝓻𝓸 ❤️"] text: String
    ) -> Result<(), Error>
{
    let mut output = String::new();
    for c in text.chars()
    {
        output.push(freaktionary(c));
    }
    ctx.say(format!("{output}"));
    Ok(())
}

//pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
//    command.name("freaky").description("get 𝓯𝓻𝓮𝓪𝓴𝔂 𝓿𝓻𝓸 ❤️")
//    .create_option(|option| {
//        option
//            .name("text")
//            .description("the text to freakify")
//            .kind(CommandOptionType::String)
//            .required(true)
//        });
//    return command;
//}

# butcher

### Vim plugin to quickly parse strings into arrays.
### It is painful to write arrays in any programming language, so butcher makes it easy for you.


## Installation
With vim-plug:
```
Plug "zim0369/butcher"
```
Add this to your vimrc:
```
noremap <leader>q :'<,'>. !xargs $HOME/.vim/plugged/butcher/target/release/butcher<CR>
```


## Usage
Suppose you have this line in your program that you want to turn into an array of strings.
```
Turn this into an array of strings
```
Now visually select the entire line and press `<leader>q`.

The above line gets replace with this beautiful array :
```
["Turn", "this", "into", "an", "array", "of", "strings"]
```

Array of characters?
Suppose you have this word in your program that you want to turn into an array of characters.
```
Characters
```
Now visually select the entire line and press `<leader>q`.

The above line gets replace with this beautiful array :
```
['C', 'h', 'a', 'r', 'a', 'c', 't', 'e', 'r', 's']
```

You can go ahead and change the keybindings.


$file = "node_modules/@vue/apollo-composable/dist/index.esm.js"
$content = Get-Content $file 
$out = @()
$out += $content[0..540]
$out += $content[541] = "nextTick(() => start());"
$out += $content[542..794] 
$out += $content[795] = "nextTick(() => start());"
$out += $content[796..($content.count -1)]
$out | out-file -Encoding utf8 $file
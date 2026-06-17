<?php
if(isset($_POST['html']) && isset($_POST['name'])){
    $name = preg_replace('/[^a-zA-Z0-9_\x{0600}-\x{06FF}]/u', '_', $_POST['name']);
    $html = $_POST['html'];
    file_put_contents("apps/$name.html", $html);
    echo "http://syria-platform.org/apps/$name.html";
}
?>

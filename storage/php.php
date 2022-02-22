<?php

$matice = [
	array("jmeno" => "premek", "prijmeni" => "vacul"),
	array("jmeno" => "jirka", "prijmeni" => "silhi")
];

foreach($matice as $key => $value){
	echo "<b>".$key.":</b><br>";
	foreach($value as $k => $val){
		echo $k.": ".$val."<br>";	
	}
}

?>
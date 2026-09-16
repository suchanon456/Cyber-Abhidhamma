<?php
require_once __DIR__ . '/../src/CetasikaVector.php';
require_once __DIR__ . '/../src/VattapadaEngine.php';

// 1. สร้าง vector ที่มีสภาวะกุศลสูง
$vector = new CetasikaVector();
$vector->activate(CetasikaVector::PHASSA, 0.9);
$vector->activate(CetasikaVector::CETANA, 0.8);
$vector->activate(27, 0.9); // saddha
$vector->activate(28, 0.8); // sati
$vector->activate(29, 0.7); // hiri
$vector->activate(30, 0.7); // ottappa
$vector->activate(31, 0.8); // alobha
$vector->activate(32, 0.8); // adosa
$vector->activate(42, 0.9); // sammavaca
$vector->activate(47, 0.9); // panna

// 2. โหลด engine
$engine = new VattapadaEngine(
    __DIR__ . '/../src/Vattapada-7.json',
    $vector
);

// 3. แสดงผล
echo "=== Vattapada 7 Scores ===\n";
foreach ($engine->evaluateAll() as $id => $score) {
    printf("  %-10s : %.3f\n", $id, $score);
}
printf("\n  Indra Score : %.3f\n", $engine->indraScore());

<?php
/**
 * vattapada_demo.php — Runnable demo for VattapadaEngine.
 *
 * Run: php examples/vattapada_demo.php
 */

require_once __DIR__ . '/../legacy-php/CetasikaVector.php';
require_once __DIR__ . '/../legacy-php/VattapadaEngine.php';

echo "=== Vattapada 7 Demo ===\n";
echo "Demonstrating the Vattapada scoring engine.\n\n";

// Create a vector with mixed wholesome/unwholesome states
$vector = new CetasikaVector();
$vector->activate(0, 0.9);   // phassa
$vector->activate(3, 0.8);   // cetana
$vector->activate(27, 0.9);  // saddha
$vector->activate(28, 0.8);  // sati
$vector->activate(29, 0.7);  // hiri
$vector->activate(30, 0.7);  // ottappa
$vector->activate(31, 0.8);  // alobha
$vector->activate(32, 0.8);  // adosa
$vector->activate(42, 0.9);  // sammavaca
$vector->activate(47, 0.9);  // panna

$engine = new VattapadaEngine(
    __DIR__ . '/../src/Vattapada-7.json',
    $vector
);

echo "=== Vattapada 7 Scores ===\n";
foreach ($engine->evaluateAll() as $id => $score) {
    printf("  %-10s : %.3f\n", $id, $score);
}

printf("\n  Indra Score : %.3f\n", $engine->indraScore());

echo "\n=== Done ===\n";

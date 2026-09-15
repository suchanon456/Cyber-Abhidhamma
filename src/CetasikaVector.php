<?php
/**
 * CetasikaVector — 52-dimensional behavioral state vector
 * Maps Abhidhamma cetasikas to computable dimensions.
 */

class CetasikaVector
{
    private array $vector;

    public function __construct()
    {
        // 52 dims — all cetasikas, initialized inactive
        $this->vector = array_fill(0, 52, 0);
    }

    public function activate(int $index, float $weight = 1.0): void
    {
        if ($index >= 0 && $index < 52) {
            $this->vector[$index] = $weight;
        }
    }

    public function toArray(): array
    {
        return $this->vector;
    }
}

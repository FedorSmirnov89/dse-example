package dse_example;

import org.opt4j.core.Objectives;
import org.opt4j.core.Objective.Sign;
import org.opt4j.core.problem.Evaluator;

public class ConfigEvaluator implements Evaluator<Configuration> {

    @Override
    public Objectives evaluate(Configuration phenotype) {
        int cost = fakeCostEval(phenotype);
        int performance = fakePerfEval(phenotype);

        Objectives obj = new Objectives();
        obj.add("cost", Sign.MIN, cost);
        obj.add("runtime", Sign.MIN, performance);

        return obj;
    }

    private int fakeCostEval(Configuration phenotype) {
        return switch (phenotype.getCoreNumber()) {
            case _2 -> 2;
            case _4 -> 4;
            case _8 -> 8;
            case _16 -> 16;
        };
    }

    private int fakePerfEval(Configuration phenotype) {
        return switch (phenotype.getMemorySize()) {
            case _4_GB -> 100;
            case _8_GB -> 80;
            case _16_GB -> 70;
            case _32_GB -> 65;
            case _64_GB -> 62;
        };
    }
}

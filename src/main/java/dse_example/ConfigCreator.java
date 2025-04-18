package dse_example;

import java.util.Random;

import org.opt4j.core.genotype.IntegerBounds;
import org.opt4j.core.genotype.IntegerGenotype;
import org.opt4j.core.problem.Creator;

import com.google.inject.Inject;

public class ConfigCreator implements Creator<IntegerGenotype> {

    private final Random random;

    @Inject
    public ConfigCreator(Random rand) {
        this.random = rand;
    }

    @Override
    public IntegerGenotype create() {

        // we will have 3 chromosomes:
        // - first one for cpu cores going from 1 to 4, representing {2, 4, 8, 16} cores
        // - second for GB of RAM, going from 1 to 5, representing {4, 8, 16, 32, 64}
        // ram GB
        // - third for the type of disk, going from 1 to 3, representing {"HDD", "SSD",
        // "NVMe"}

        int[] lowerBounds = { 1, 1, 1 };
        int[] upperBounds = { 4, 5, 3 };
        var integerBounds = new IntegerBounds(lowerBounds, upperBounds);

        var geno = new IntegerGenotype(integerBounds);
        geno.init(this.random, 3);
        return geno;
    }

}

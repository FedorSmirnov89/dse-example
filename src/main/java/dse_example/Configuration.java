package dse_example;

/**
 * Represents a particular configuration of the system.
 * 
 * In terms of the GA optimization, this is the 'phenotype' defining the fitness
 * of the considered solution.
 */
public class Configuration {

    public static enum DiskType {
        HDD,
        SSD,
        NVMe
    }

    public static enum MemorySize {
        _4_GB,
        _8_GB,
        _16_GB,
        _32_GB,
        _64_GB,
    }

    public static enum CoreNumber {
        _2,
        _4,
        _8,
        _16,
    }

    private final DiskType diskType;
    private final MemorySize memorySize;
    private final CoreNumber coreNumber;

    public Configuration(int geneNumCores, int geneMemorySize, int geneDistType) {
        this.coreNumber = switch (geneNumCores) {
            case 1 -> CoreNumber._2;
            case 2 -> CoreNumber._4;
            case 3 -> CoreNumber._8;
            case 4 -> CoreNumber._16;
            default -> throw new RuntimeException("invalid gene encoding core number: " + geneNumCores);
        };

        this.memorySize = switch (geneMemorySize) {
            case 1 -> MemorySize._4_GB;
            case 2 -> MemorySize._8_GB;
            case 3 -> MemorySize._16_GB;
            case 4 -> MemorySize._32_GB;
            case 5 -> MemorySize._64_GB;
            default -> throw new RuntimeException("invalid gene encoding memory size: " + geneMemorySize);
        };

        this.diskType = switch (geneDistType) {
            case 1 -> DiskType.HDD;
            case 2 -> DiskType.SSD;
            case 3 -> DiskType.NVMe;
            default -> throw new RuntimeException("invalid gene encoding disk type: " + geneDistType);
        };

    }

    public CoreNumber getCoreNumber() {
        return coreNumber;
    }

    public DiskType getDiskType() {
        return diskType;
    }

    public MemorySize getMemorySize() {
        return memorySize;
    }

    @Override
    public String toString() {
        return "memorySize: " + this.memorySize + " ; coreNumber: " + this.coreNumber + " ; diskType: " + diskType;
    }
}

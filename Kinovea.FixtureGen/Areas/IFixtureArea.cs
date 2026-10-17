namespace Kinovea.FixtureGen.Areas
{
    /// <summary>
    /// A fixture area runs the original implementation over one case and writes its outputs.
    /// </summary>
    public interface IFixtureArea
    {
        string Name { get; }
        void Generate(FixtureCase fixtureCase, string outDir);
    }
}

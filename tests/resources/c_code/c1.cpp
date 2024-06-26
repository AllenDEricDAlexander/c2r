class AbstractK {
public:
double r;
double p;
AbstractK(const double & r, const double & p) : r(r), p(p) {};
virtual const bool isInside() const = 0;
virtual const bool overlap() const = 0;
}
class K2D : public AbstractK {
public:
K2D(const double & r, const double & p) : AbstractK(r, p) {};
virtual const bool overlap() const override;
}
class K3D : public AbstractK {
public:
K3D(const double & r, const double & p) : AbstractK(r, p) {};
virtual const bool overlap() const override;
class SquareK {
public:
SquareK(const double & r, const double & p) : K2D(c, r) {};
virtual const bool isInside() const override;
}
class CubeK {
public:
CubeK(const double & r, const double & p) : K3D(c, r) {};
virtual const bool isInside() const override;
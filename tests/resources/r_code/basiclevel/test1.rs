
trait AbstractK {
    fn new(r: f64, p: f64) -> Self;
    fn is_inside(&self) -> bool;
    fn overlap(&self) -> bool;
    fn r(&self) -> f64;
    fn p(&self) -> f64;
}

struct K2D {
    r: f64,
    p: f64,
}

impl AbstractK for K2D {
    fn new(r: f64, p: f64) -> Self {
        K2D { r, p }
    }

    fn is_inside(&self) -> bool {
        // Default implementation or override in derived classes
        false
    }

    fn overlap(&self) -> bool {
        // Override in derived classes
        false
    }

    fn r(&self) -> f64 {
        self.r
    }

    fn p(&self) -> f64 {
        self.p
    }
}

struct K3D {
    r: f64,
    p: f64,
}

impl AbstractK for K3D {
    fn new(r: f64, p: f64) -> Self {
        K3D { r, p }
    }

    fn is_inside(&self) -> bool {
        // Default implementation or override in derived classes
        false
    }

    fn overlap(&self) -> bool {
        // Override in derived classes
        false
    }

    fn r(&self) -> f64 {
        self.r
    }

    fn p(&self) -> f64 {
        self.p
    }
}

struct SquareK {
    k2d: K2D,
}

impl SquareK {
    fn new(r: f64, p: f64) -> Self {
        SquareK {
            k2d: K2D::new(r, p),
        }
    }
}

impl AbstractK for SquareK {
    fn is_inside(&self) -> bool {
        // Override implementation
        false
    }

    fn overlap(&self) -> bool {
        // Inherited from K2D
        self.k2d.overlap()
    }

    fn r(&self) -> f64 {
        self.k2d.r()
    }

    fn p(&self) -> f64 {
        self.k2d.p()
    }
}

struct CubeK {
    k3d: K3D,
}

impl CubeK {
    fn new(r: f64, p: f64) -> Self {
        CubeK {
            k3d: K3D::new(r, p),
        }
    }
}

impl AbstractK for CubeK {
    fn is_inside(&self) -> bool {
        // Override implementation
        false
    }

    fn overlap(&self) -> bool {
        // Inherited from K3D
        self.k3d.overlap()
    }

    fn r(&self) -> f64 {
        self.k3d.r()
    }

    fn p(&self) -> f64 {
        self.k3d.p()
    }
}

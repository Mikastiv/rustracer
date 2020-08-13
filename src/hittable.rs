use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub enum Hittable {
    Sphere {
        center: Vec3,
        radius: f64,
        material: Material,
    },
    MovingSphere {
        center0: Vec3,
        center1: Vec3,
        time0: f64,
        time1: f64,
        radius: f64,
        material: Material,
    },
}

impl Hittable {
    pub fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        match self {
            Self::Sphere {
                center,
                radius,
                material,
            } => {
                let oc = ray.origin - *center;
                let a = ray.dir.length_sq();
                let half_b = oc.dot(ray.dir);
                let c = oc.length_sq() - radius.powi(2);
                let discriminant = (half_b * half_b) - (a * c);

                if discriminant > 0.0 {
                    let root = discriminant.sqrt();

                    {
                        let temp = (-half_b - root) / a;
                        if temp < t_max && temp > t_min {
                            let t = temp;
                            let point = ray.at(t);
                            let outward_normal = ((point - *center) / *radius).normalize();
                            let (front_face, normal) =
                                Intersection::get_face_normal(ray, outward_normal);
                            return Some(Intersection {
                                point,
                                normal,
                                t,
                                front_face,
                                material,
                            });
                        }
                    }

                    {
                        let temp = (-half_b + root) / a;
                        if temp < t_max && temp > t_min {
                            let t = temp;
                            let point = ray.at(t);
                            let outward_normal = ((point - *center) / *radius).normalize();
                            let (front_face, normal) =
                                Intersection::get_face_normal(ray, outward_normal);
                            return Some(Intersection {
                                point,
                                normal,
                                t,
                                front_face,
                                material,
                            });
                        }
                    }
                }

                None
            }
            Self::MovingSphere {
                center0,
                center1,
                time0,
                time1,
                radius,
                material,
            } => {
                let oc = ray.origin - center(*center0, *center1, *time0, *time1, ray.time);
                let a = ray.dir.length_sq();
                let half_b = oc.dot(ray.dir);
                let c = oc.length_sq() - radius.powi(2);
                let discriminant = (half_b * half_b) - (a * c);

                if discriminant > 0.0 {
                    let root = discriminant.sqrt();

                    {
                        let temp = (-half_b - root) / a;
                        if temp < t_max && temp > t_min {
                            let t = temp;
                            let point = ray.at(t);
                            let outward_normal = ((point
                                - center(*center0, *center1, *time0, *time1, ray.time))
                                / *radius)
                                .normalize();
                            let (front_face, normal) =
                                Intersection::get_face_normal(ray, outward_normal);
                            return Some(Intersection {
                                point,
                                normal,
                                t,
                                front_face,
                                material,
                            });
                        }
                    }

                    {
                        let temp = (-half_b + root) / a;
                        if temp < t_max && temp > t_min {
                            let t = temp;
                            let point = ray.at(t);
                            let outward_normal = ((point
                                - center(*center0, *center1, *time0, *time1, ray.time))
                                / *radius)
                                .normalize();
                            let (front_face, normal) =
                                Intersection::get_face_normal(ray, outward_normal);
                            return Some(Intersection {
                                point,
                                normal,
                                t,
                                front_face,
                                material,
                            });
                        }
                    }
                }

                None
            }
        }
    }
}

fn center(center0: Vec3, center1: Vec3, t0: f64, t1: f64, time: f64) -> Vec3 {
    center0 + ((time - t0) / (t1 - t0)) * (center1 - center0)
}

pub struct HittableList {
    objects: Vec<Hittable>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Hittable) {
        self.objects.push(object);
    }

    pub fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        let mut closest = t_max;
        let mut intersection_out = None;

        for object in self.objects.iter() {
            let intersection = object.hit(ray, t_min, closest);

            intersection_out = match intersection {
                Some(hit_record) => {
                    closest = hit_record.t;
                    Some(hit_record)
                }
                None => intersection_out,
            }
        }

        intersection_out
    }
}

pub struct Intersection<'a> {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: &'a Material,
}

impl Intersection<'_> {
    #[inline]
    pub fn get_face_normal(ray: Ray, outward_normal: Vec3) -> (bool, Vec3) {
        let front_face = ray.dir.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        (front_face, normal)
    }
}

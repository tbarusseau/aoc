pub trait GetIndexOfMax {
    fn get_index_of_max<T>(self) -> Option<usize>
    where
        Self: Iterator<Item = T>,
        T: Ord;
}

impl<It> GetIndexOfMax for It
where
    It: Iterator + Sized,
{
    fn get_index_of_max<T>(self) -> Option<usize>
    where
        Self: Iterator<Item = T>,
        T: Ord,
    {
        self.enumerate()
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(index, _)| index)
    }
}

// pub trait GetIndexOfMin {
//     fn get_index_of_min<T>(self) -> Option<usize>
//     where
//         Self: Iterator<Item = T>,
//         T: Ord;
// }

// impl<It> GetIndexOfMin for It
// where
//     It: Iterator + Sized,
// {
//     fn get_index_of_min<T>(self) -> Option<usize>
//     where
//         Self: Iterator<Item = T>,
//         T: Ord,
//     {
//         self.enumerate()
//             .min_by(|(_, a), (_, b)| a.cmp(b))
//             .map(|(index, _)| index)
//     }
// }

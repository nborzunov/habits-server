import {
    FiCheck,
    FiChevronLeft,
    FiChevronRight,
    FiChevronsLeft,
    FiChevronsRight,
    FiChevronsUp,
    FiEdit3,
    FiEye,
    FiEyeOff,
    FiHome,
    FiInbox,
    FiMenu,
    FiPieChart,
    FiPlus,
} from 'react-icons/fi';
import { HiFire } from 'react-icons/hi';
import { RxCross2 } from 'react-icons/rx';
import { IoIosNotificationsOutline } from 'react-icons/io';
import { MdDeleteOutline, MdOutlineArchive } from 'react-icons/all';

const Icons = {
    Inbox: FiInbox,
    Complete: FiCheck,
    Cross: RxCross2,
    ArrowTop: FiChevronsUp,
    Streak: HiFire,
    Dashboard: FiHome,
    Menu: FiMenu,
    Notifications: IoIosNotificationsOutline,
    Chart: FiPieChart,
    Add: FiPlus,
    Edit: FiEdit3,
    Delete: MdDeleteOutline,
    Archive: MdOutlineArchive,
    Show: FiEye,
    Hide: FiEyeOff,
    Back: FiChevronLeft,
    Left: FiChevronLeft,
    Right: FiChevronRight,
    LeftDouble: FiChevronsLeft,
    RightDouble: FiChevronsRight,
};

// TODO: write proxy for icons
// const Icons: {[key: string]: typeof Icon} = new Proxy(IconNames, {
//     get(target, name) {
//         return <Icon as={target[name]} />;
//     },
// });

export default Icons;
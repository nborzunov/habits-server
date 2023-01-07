import {
    FiCheck,
    FiChevronsUp,
    FiEdit3,
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
};

export default Icons;

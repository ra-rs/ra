#[doc = "Register `ADOVFCHSCR0` writer"]
pub struct W(crate::W<ADOVFCHSCR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADOVFCHSCR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ADOVFCHSCR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADOVFCHSCR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Analog Channel n: Overflow Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFCHC0_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADOVFCHSR0.OVFCHFn is cleared"]
    _1 = 1,
}
impl From<OVFCHC0_AW> for bool {
    #[inline(always)]
    fn from(variant: OVFCHC0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFCHC0` writer - Analog Channel n: Overflow Flag Clear"]
pub type OVFCHC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADOVFCHSCR0_SPEC, OVFCHC0_AW, O>;
impl<'a, const O: u8> OVFCHC0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVFCHC0_AW::_0)
    }
    #[doc = "ADOVFCHSR0.OVFCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVFCHC0_AW::_1)
    }
}
#[doc = "Analog Channel n: Overflow Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFCHC1_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADOVFCHSR0.OVFCHFn is cleared"]
    _1 = 1,
}
impl From<OVFCHC1_AW> for bool {
    #[inline(always)]
    fn from(variant: OVFCHC1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFCHC1` writer - Analog Channel n: Overflow Flag Clear"]
pub type OVFCHC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADOVFCHSCR0_SPEC, OVFCHC1_AW, O>;
impl<'a, const O: u8> OVFCHC1_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVFCHC1_AW::_0)
    }
    #[doc = "ADOVFCHSR0.OVFCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVFCHC1_AW::_1)
    }
}
#[doc = "Analog Channel n: Overflow Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFCHC2_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADOVFCHSR0.OVFCHFn is cleared"]
    _1 = 1,
}
impl From<OVFCHC2_AW> for bool {
    #[inline(always)]
    fn from(variant: OVFCHC2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFCHC2` writer - Analog Channel n: Overflow Flag Clear"]
pub type OVFCHC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADOVFCHSCR0_SPEC, OVFCHC2_AW, O>;
impl<'a, const O: u8> OVFCHC2_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVFCHC2_AW::_0)
    }
    #[doc = "ADOVFCHSR0.OVFCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVFCHC2_AW::_1)
    }
}
#[doc = "Analog Channel n: Overflow Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFCHC3_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADOVFCHSR0.OVFCHFn is cleared"]
    _1 = 1,
}
impl From<OVFCHC3_AW> for bool {
    #[inline(always)]
    fn from(variant: OVFCHC3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFCHC3` writer - Analog Channel n: Overflow Flag Clear"]
pub type OVFCHC3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADOVFCHSCR0_SPEC, OVFCHC3_AW, O>;
impl<'a, const O: u8> OVFCHC3_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVFCHC3_AW::_0)
    }
    #[doc = "ADOVFCHSR0.OVFCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVFCHC3_AW::_1)
    }
}
#[doc = "Analog Channel n: Overflow Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFCHC4_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADOVFCHSR0.OVFCHFn is cleared"]
    _1 = 1,
}
impl From<OVFCHC4_AW> for bool {
    #[inline(always)]
    fn from(variant: OVFCHC4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFCHC4` writer - Analog Channel n: Overflow Flag Clear"]
pub type OVFCHC4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADOVFCHSCR0_SPEC, OVFCHC4_AW, O>;
impl<'a, const O: u8> OVFCHC4_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVFCHC4_AW::_0)
    }
    #[doc = "ADOVFCHSR0.OVFCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVFCHC4_AW::_1)
    }
}
#[doc = "Analog Channel n: Overflow Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFCHC5_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADOVFCHSR0.OVFCHFn is cleared"]
    _1 = 1,
}
impl From<OVFCHC5_AW> for bool {
    #[inline(always)]
    fn from(variant: OVFCHC5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFCHC5` writer - Analog Channel n: Overflow Flag Clear"]
pub type OVFCHC5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADOVFCHSCR0_SPEC, OVFCHC5_AW, O>;
impl<'a, const O: u8> OVFCHC5_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVFCHC5_AW::_0)
    }
    #[doc = "ADOVFCHSR0.OVFCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVFCHC5_AW::_1)
    }
}
#[doc = "Analog Channel n: Overflow Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFCHC6_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADOVFCHSR0.OVFCHFn is cleared"]
    _1 = 1,
}
impl From<OVFCHC6_AW> for bool {
    #[inline(always)]
    fn from(variant: OVFCHC6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFCHC6` writer - Analog Channel n: Overflow Flag Clear"]
pub type OVFCHC6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADOVFCHSCR0_SPEC, OVFCHC6_AW, O>;
impl<'a, const O: u8> OVFCHC6_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVFCHC6_AW::_0)
    }
    #[doc = "ADOVFCHSR0.OVFCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVFCHC6_AW::_1)
    }
}
#[doc = "Analog Channel n: Overflow Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFCHC7_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADOVFCHSR0.OVFCHFn is cleared"]
    _1 = 1,
}
impl From<OVFCHC7_AW> for bool {
    #[inline(always)]
    fn from(variant: OVFCHC7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFCHC7` writer - Analog Channel n: Overflow Flag Clear"]
pub type OVFCHC7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADOVFCHSCR0_SPEC, OVFCHC7_AW, O>;
impl<'a, const O: u8> OVFCHC7_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVFCHC7_AW::_0)
    }
    #[doc = "ADOVFCHSR0.OVFCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVFCHC7_AW::_1)
    }
}
#[doc = "Analog Channel n: Overflow Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFCHC8_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADOVFCHSR0.OVFCHFn is cleared"]
    _1 = 1,
}
impl From<OVFCHC8_AW> for bool {
    #[inline(always)]
    fn from(variant: OVFCHC8_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFCHC8` writer - Analog Channel n: Overflow Flag Clear"]
pub type OVFCHC8_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADOVFCHSCR0_SPEC, OVFCHC8_AW, O>;
impl<'a, const O: u8> OVFCHC8_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVFCHC8_AW::_0)
    }
    #[doc = "ADOVFCHSR0.OVFCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVFCHC8_AW::_1)
    }
}
#[doc = "Analog Channel n: Overflow Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFCHC9_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADOVFCHSR0.OVFCHFn is cleared"]
    _1 = 1,
}
impl From<OVFCHC9_AW> for bool {
    #[inline(always)]
    fn from(variant: OVFCHC9_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFCHC9` writer - Analog Channel n: Overflow Flag Clear"]
pub type OVFCHC9_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADOVFCHSCR0_SPEC, OVFCHC9_AW, O>;
impl<'a, const O: u8> OVFCHC9_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVFCHC9_AW::_0)
    }
    #[doc = "ADOVFCHSR0.OVFCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVFCHC9_AW::_1)
    }
}
#[doc = "Analog Channel n: Overflow Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFCHC10_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADOVFCHSR0.OVFCHFn is cleared"]
    _1 = 1,
}
impl From<OVFCHC10_AW> for bool {
    #[inline(always)]
    fn from(variant: OVFCHC10_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFCHC10` writer - Analog Channel n: Overflow Flag Clear"]
pub type OVFCHC10_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADOVFCHSCR0_SPEC, OVFCHC10_AW, O>;
impl<'a, const O: u8> OVFCHC10_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVFCHC10_AW::_0)
    }
    #[doc = "ADOVFCHSR0.OVFCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVFCHC10_AW::_1)
    }
}
#[doc = "Analog Channel n: Overflow Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFCHC11_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADOVFCHSR0.OVFCHFn is cleared"]
    _1 = 1,
}
impl From<OVFCHC11_AW> for bool {
    #[inline(always)]
    fn from(variant: OVFCHC11_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFCHC11` writer - Analog Channel n: Overflow Flag Clear"]
pub type OVFCHC11_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADOVFCHSCR0_SPEC, OVFCHC11_AW, O>;
impl<'a, const O: u8> OVFCHC11_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVFCHC11_AW::_0)
    }
    #[doc = "ADOVFCHSR0.OVFCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVFCHC11_AW::_1)
    }
}
#[doc = "Analog Channel n: Overflow Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFCHC12_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADOVFCHSR0.OVFCHFn is cleared"]
    _1 = 1,
}
impl From<OVFCHC12_AW> for bool {
    #[inline(always)]
    fn from(variant: OVFCHC12_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFCHC12` writer - Analog Channel n: Overflow Flag Clear"]
pub type OVFCHC12_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADOVFCHSCR0_SPEC, OVFCHC12_AW, O>;
impl<'a, const O: u8> OVFCHC12_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVFCHC12_AW::_0)
    }
    #[doc = "ADOVFCHSR0.OVFCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVFCHC12_AW::_1)
    }
}
#[doc = "Analog Channel n: Overflow Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFCHC13_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADOVFCHSR0.OVFCHFn is cleared"]
    _1 = 1,
}
impl From<OVFCHC13_AW> for bool {
    #[inline(always)]
    fn from(variant: OVFCHC13_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFCHC13` writer - Analog Channel n: Overflow Flag Clear"]
pub type OVFCHC13_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADOVFCHSCR0_SPEC, OVFCHC13_AW, O>;
impl<'a, const O: u8> OVFCHC13_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVFCHC13_AW::_0)
    }
    #[doc = "ADOVFCHSR0.OVFCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVFCHC13_AW::_1)
    }
}
#[doc = "Analog Channel n: Overflow Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFCHC14_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADOVFCHSR0.OVFCHFn is cleared"]
    _1 = 1,
}
impl From<OVFCHC14_AW> for bool {
    #[inline(always)]
    fn from(variant: OVFCHC14_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFCHC14` writer - Analog Channel n: Overflow Flag Clear"]
pub type OVFCHC14_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADOVFCHSCR0_SPEC, OVFCHC14_AW, O>;
impl<'a, const O: u8> OVFCHC14_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVFCHC14_AW::_0)
    }
    #[doc = "ADOVFCHSR0.OVFCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVFCHC14_AW::_1)
    }
}
#[doc = "Analog Channel n: Overflow Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFCHC15_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADOVFCHSR0.OVFCHFn is cleared"]
    _1 = 1,
}
impl From<OVFCHC15_AW> for bool {
    #[inline(always)]
    fn from(variant: OVFCHC15_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFCHC15` writer - Analog Channel n: Overflow Flag Clear"]
pub type OVFCHC15_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADOVFCHSCR0_SPEC, OVFCHC15_AW, O>;
impl<'a, const O: u8> OVFCHC15_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVFCHC15_AW::_0)
    }
    #[doc = "ADOVFCHSR0.OVFCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVFCHC15_AW::_1)
    }
}
#[doc = "Analog Channel n: Overflow Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFCHC16_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADOVFCHSR0.OVFCHFn is cleared"]
    _1 = 1,
}
impl From<OVFCHC16_AW> for bool {
    #[inline(always)]
    fn from(variant: OVFCHC16_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFCHC16` writer - Analog Channel n: Overflow Flag Clear"]
pub type OVFCHC16_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADOVFCHSCR0_SPEC, OVFCHC16_AW, O>;
impl<'a, const O: u8> OVFCHC16_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVFCHC16_AW::_0)
    }
    #[doc = "ADOVFCHSR0.OVFCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVFCHC16_AW::_1)
    }
}
#[doc = "Analog Channel n: Overflow Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFCHC17_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADOVFCHSR0.OVFCHFn is cleared"]
    _1 = 1,
}
impl From<OVFCHC17_AW> for bool {
    #[inline(always)]
    fn from(variant: OVFCHC17_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFCHC17` writer - Analog Channel n: Overflow Flag Clear"]
pub type OVFCHC17_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADOVFCHSCR0_SPEC, OVFCHC17_AW, O>;
impl<'a, const O: u8> OVFCHC17_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVFCHC17_AW::_0)
    }
    #[doc = "ADOVFCHSR0.OVFCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVFCHC17_AW::_1)
    }
}
#[doc = "Analog Channel n: Overflow Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFCHC18_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADOVFCHSR0.OVFCHFn is cleared"]
    _1 = 1,
}
impl From<OVFCHC18_AW> for bool {
    #[inline(always)]
    fn from(variant: OVFCHC18_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFCHC18` writer - Analog Channel n: Overflow Flag Clear"]
pub type OVFCHC18_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADOVFCHSCR0_SPEC, OVFCHC18_AW, O>;
impl<'a, const O: u8> OVFCHC18_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVFCHC18_AW::_0)
    }
    #[doc = "ADOVFCHSR0.OVFCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVFCHC18_AW::_1)
    }
}
#[doc = "Analog Channel n: Overflow Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFCHC19_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADOVFCHSR0.OVFCHFn is cleared"]
    _1 = 1,
}
impl From<OVFCHC19_AW> for bool {
    #[inline(always)]
    fn from(variant: OVFCHC19_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFCHC19` writer - Analog Channel n: Overflow Flag Clear"]
pub type OVFCHC19_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADOVFCHSCR0_SPEC, OVFCHC19_AW, O>;
impl<'a, const O: u8> OVFCHC19_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVFCHC19_AW::_0)
    }
    #[doc = "ADOVFCHSR0.OVFCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVFCHC19_AW::_1)
    }
}
#[doc = "Analog Channel n: Overflow Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFCHC20_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADOVFCHSR0.OVFCHFn is cleared"]
    _1 = 1,
}
impl From<OVFCHC20_AW> for bool {
    #[inline(always)]
    fn from(variant: OVFCHC20_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFCHC20` writer - Analog Channel n: Overflow Flag Clear"]
pub type OVFCHC20_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADOVFCHSCR0_SPEC, OVFCHC20_AW, O>;
impl<'a, const O: u8> OVFCHC20_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVFCHC20_AW::_0)
    }
    #[doc = "ADOVFCHSR0.OVFCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVFCHC20_AW::_1)
    }
}
#[doc = "Analog Channel n: Overflow Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFCHC21_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADOVFCHSR0.OVFCHFn is cleared"]
    _1 = 1,
}
impl From<OVFCHC21_AW> for bool {
    #[inline(always)]
    fn from(variant: OVFCHC21_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFCHC21` writer - Analog Channel n: Overflow Flag Clear"]
pub type OVFCHC21_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADOVFCHSCR0_SPEC, OVFCHC21_AW, O>;
impl<'a, const O: u8> OVFCHC21_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVFCHC21_AW::_0)
    }
    #[doc = "ADOVFCHSR0.OVFCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVFCHC21_AW::_1)
    }
}
#[doc = "Analog Channel n: Overflow Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFCHC22_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADOVFCHSR0.OVFCHFn is cleared"]
    _1 = 1,
}
impl From<OVFCHC22_AW> for bool {
    #[inline(always)]
    fn from(variant: OVFCHC22_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFCHC22` writer - Analog Channel n: Overflow Flag Clear"]
pub type OVFCHC22_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADOVFCHSCR0_SPEC, OVFCHC22_AW, O>;
impl<'a, const O: u8> OVFCHC22_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVFCHC22_AW::_0)
    }
    #[doc = "ADOVFCHSR0.OVFCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVFCHC22_AW::_1)
    }
}
#[doc = "Analog Channel n: Overflow Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFCHC23_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADOVFCHSR0.OVFCHFn is cleared"]
    _1 = 1,
}
impl From<OVFCHC23_AW> for bool {
    #[inline(always)]
    fn from(variant: OVFCHC23_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFCHC23` writer - Analog Channel n: Overflow Flag Clear"]
pub type OVFCHC23_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADOVFCHSCR0_SPEC, OVFCHC23_AW, O>;
impl<'a, const O: u8> OVFCHC23_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVFCHC23_AW::_0)
    }
    #[doc = "ADOVFCHSR0.OVFCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVFCHC23_AW::_1)
    }
}
#[doc = "Analog Channel n: Overflow Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFCHC24_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADOVFCHSR0.OVFCHFn is cleared"]
    _1 = 1,
}
impl From<OVFCHC24_AW> for bool {
    #[inline(always)]
    fn from(variant: OVFCHC24_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFCHC24` writer - Analog Channel n: Overflow Flag Clear"]
pub type OVFCHC24_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADOVFCHSCR0_SPEC, OVFCHC24_AW, O>;
impl<'a, const O: u8> OVFCHC24_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVFCHC24_AW::_0)
    }
    #[doc = "ADOVFCHSR0.OVFCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVFCHC24_AW::_1)
    }
}
#[doc = "Analog Channel n: Overflow Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFCHC25_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADOVFCHSR0.OVFCHFn is cleared"]
    _1 = 1,
}
impl From<OVFCHC25_AW> for bool {
    #[inline(always)]
    fn from(variant: OVFCHC25_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFCHC25` writer - Analog Channel n: Overflow Flag Clear"]
pub type OVFCHC25_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADOVFCHSCR0_SPEC, OVFCHC25_AW, O>;
impl<'a, const O: u8> OVFCHC25_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVFCHC25_AW::_0)
    }
    #[doc = "ADOVFCHSR0.OVFCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVFCHC25_AW::_1)
    }
}
#[doc = "Analog Channel n: Overflow Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFCHC26_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADOVFCHSR0.OVFCHFn is cleared"]
    _1 = 1,
}
impl From<OVFCHC26_AW> for bool {
    #[inline(always)]
    fn from(variant: OVFCHC26_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFCHC26` writer - Analog Channel n: Overflow Flag Clear"]
pub type OVFCHC26_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADOVFCHSCR0_SPEC, OVFCHC26_AW, O>;
impl<'a, const O: u8> OVFCHC26_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVFCHC26_AW::_0)
    }
    #[doc = "ADOVFCHSR0.OVFCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVFCHC26_AW::_1)
    }
}
#[doc = "Analog Channel n: Overflow Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFCHC27_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADOVFCHSR0.OVFCHFn is cleared"]
    _1 = 1,
}
impl From<OVFCHC27_AW> for bool {
    #[inline(always)]
    fn from(variant: OVFCHC27_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFCHC27` writer - Analog Channel n: Overflow Flag Clear"]
pub type OVFCHC27_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADOVFCHSCR0_SPEC, OVFCHC27_AW, O>;
impl<'a, const O: u8> OVFCHC27_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVFCHC27_AW::_0)
    }
    #[doc = "ADOVFCHSR0.OVFCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVFCHC27_AW::_1)
    }
}
#[doc = "Analog Channel n: Overflow Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFCHC28_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: ADOVFCHSR0.OVFCHFn is cleared"]
    _1 = 1,
}
impl From<OVFCHC28_AW> for bool {
    #[inline(always)]
    fn from(variant: OVFCHC28_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFCHC28` writer - Analog Channel n: Overflow Flag Clear"]
pub type OVFCHC28_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADOVFCHSCR0_SPEC, OVFCHC28_AW, O>;
impl<'a, const O: u8> OVFCHC28_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVFCHC28_AW::_0)
    }
    #[doc = "ADOVFCHSR0.OVFCHFn is cleared"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVFCHC28_AW::_1)
    }
}
impl W {
    #[doc = "Bit 0 - Analog Channel n: Overflow Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ovfchc0(&mut self) -> OVFCHC0_W<0> {
        OVFCHC0_W::new(self)
    }
    #[doc = "Bit 1 - Analog Channel n: Overflow Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ovfchc1(&mut self) -> OVFCHC1_W<1> {
        OVFCHC1_W::new(self)
    }
    #[doc = "Bit 2 - Analog Channel n: Overflow Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ovfchc2(&mut self) -> OVFCHC2_W<2> {
        OVFCHC2_W::new(self)
    }
    #[doc = "Bit 3 - Analog Channel n: Overflow Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ovfchc3(&mut self) -> OVFCHC3_W<3> {
        OVFCHC3_W::new(self)
    }
    #[doc = "Bit 4 - Analog Channel n: Overflow Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ovfchc4(&mut self) -> OVFCHC4_W<4> {
        OVFCHC4_W::new(self)
    }
    #[doc = "Bit 5 - Analog Channel n: Overflow Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ovfchc5(&mut self) -> OVFCHC5_W<5> {
        OVFCHC5_W::new(self)
    }
    #[doc = "Bit 6 - Analog Channel n: Overflow Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ovfchc6(&mut self) -> OVFCHC6_W<6> {
        OVFCHC6_W::new(self)
    }
    #[doc = "Bit 7 - Analog Channel n: Overflow Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ovfchc7(&mut self) -> OVFCHC7_W<7> {
        OVFCHC7_W::new(self)
    }
    #[doc = "Bit 8 - Analog Channel n: Overflow Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ovfchc8(&mut self) -> OVFCHC8_W<8> {
        OVFCHC8_W::new(self)
    }
    #[doc = "Bit 9 - Analog Channel n: Overflow Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ovfchc9(&mut self) -> OVFCHC9_W<9> {
        OVFCHC9_W::new(self)
    }
    #[doc = "Bit 10 - Analog Channel n: Overflow Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ovfchc10(&mut self) -> OVFCHC10_W<10> {
        OVFCHC10_W::new(self)
    }
    #[doc = "Bit 11 - Analog Channel n: Overflow Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ovfchc11(&mut self) -> OVFCHC11_W<11> {
        OVFCHC11_W::new(self)
    }
    #[doc = "Bit 12 - Analog Channel n: Overflow Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ovfchc12(&mut self) -> OVFCHC12_W<12> {
        OVFCHC12_W::new(self)
    }
    #[doc = "Bit 13 - Analog Channel n: Overflow Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ovfchc13(&mut self) -> OVFCHC13_W<13> {
        OVFCHC13_W::new(self)
    }
    #[doc = "Bit 14 - Analog Channel n: Overflow Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ovfchc14(&mut self) -> OVFCHC14_W<14> {
        OVFCHC14_W::new(self)
    }
    #[doc = "Bit 15 - Analog Channel n: Overflow Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ovfchc15(&mut self) -> OVFCHC15_W<15> {
        OVFCHC15_W::new(self)
    }
    #[doc = "Bit 16 - Analog Channel n: Overflow Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ovfchc16(&mut self) -> OVFCHC16_W<16> {
        OVFCHC16_W::new(self)
    }
    #[doc = "Bit 17 - Analog Channel n: Overflow Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ovfchc17(&mut self) -> OVFCHC17_W<17> {
        OVFCHC17_W::new(self)
    }
    #[doc = "Bit 18 - Analog Channel n: Overflow Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ovfchc18(&mut self) -> OVFCHC18_W<18> {
        OVFCHC18_W::new(self)
    }
    #[doc = "Bit 19 - Analog Channel n: Overflow Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ovfchc19(&mut self) -> OVFCHC19_W<19> {
        OVFCHC19_W::new(self)
    }
    #[doc = "Bit 20 - Analog Channel n: Overflow Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ovfchc20(&mut self) -> OVFCHC20_W<20> {
        OVFCHC20_W::new(self)
    }
    #[doc = "Bit 21 - Analog Channel n: Overflow Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ovfchc21(&mut self) -> OVFCHC21_W<21> {
        OVFCHC21_W::new(self)
    }
    #[doc = "Bit 22 - Analog Channel n: Overflow Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ovfchc22(&mut self) -> OVFCHC22_W<22> {
        OVFCHC22_W::new(self)
    }
    #[doc = "Bit 23 - Analog Channel n: Overflow Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ovfchc23(&mut self) -> OVFCHC23_W<23> {
        OVFCHC23_W::new(self)
    }
    #[doc = "Bit 24 - Analog Channel n: Overflow Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ovfchc24(&mut self) -> OVFCHC24_W<24> {
        OVFCHC24_W::new(self)
    }
    #[doc = "Bit 25 - Analog Channel n: Overflow Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ovfchc25(&mut self) -> OVFCHC25_W<25> {
        OVFCHC25_W::new(self)
    }
    #[doc = "Bit 26 - Analog Channel n: Overflow Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ovfchc26(&mut self) -> OVFCHC26_W<26> {
        OVFCHC26_W::new(self)
    }
    #[doc = "Bit 27 - Analog Channel n: Overflow Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ovfchc27(&mut self) -> OVFCHC27_W<27> {
        OVFCHC27_W::new(self)
    }
    #[doc = "Bit 28 - Analog Channel n: Overflow Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ovfchc28(&mut self) -> OVFCHC28_W<28> {
        OVFCHC28_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Conversion Overflow Channel Status Clear Register 0\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adovfchscr0](index.html) module"]
pub struct ADOVFCHSCR0_SPEC;
impl crate::RegisterSpec for ADOVFCHSCR0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [adovfchscr0::W](W) writer structure"]
impl crate::Writable for ADOVFCHSCR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADOVFCHSCR0 to value 0"]
impl crate::Resettable for ADOVFCHSCR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

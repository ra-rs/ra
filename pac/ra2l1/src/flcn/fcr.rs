#[doc = "Register `FCR` reader"]
pub struct R(crate::R<FCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCR` writer"]
pub struct W(crate::W<FCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCR_SPEC>;
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
impl From<crate::W<FCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMD` reader - Software Command Setting"]
pub type CMD_R = crate::FieldReader<u8, CMD_A>;
#[doc = "Software Command Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMD_A {
    #[doc = "1: Program"]
    _0X1 = 1,
    #[doc = "3: Blank check (code flash)"]
    _0X3 = 3,
    #[doc = "4: Block erase"]
    _0X4 = 4,
    #[doc = "5: Consecutive read"]
    _0X5 = 5,
    #[doc = "6: Chip erase"]
    _0X6 = 6,
    #[doc = "11: Blank check (data flash)"]
    _0X_B = 11,
}
impl From<CMD_A> for u8 {
    #[inline(always)]
    fn from(variant: CMD_A) -> Self {
        variant as _
    }
}
impl CMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMD_A> {
        match self.bits {
            1 => Some(CMD_A::_0X1),
            3 => Some(CMD_A::_0X3),
            4 => Some(CMD_A::_0X4),
            5 => Some(CMD_A::_0X5),
            6 => Some(CMD_A::_0X6),
            11 => Some(CMD_A::_0X_B),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X1`"]
    #[inline(always)]
    pub fn is_0x1(&self) -> bool {
        *self == CMD_A::_0X1
    }
    #[doc = "Checks if the value of the field is `_0X3`"]
    #[inline(always)]
    pub fn is_0x3(&self) -> bool {
        *self == CMD_A::_0X3
    }
    #[doc = "Checks if the value of the field is `_0X4`"]
    #[inline(always)]
    pub fn is_0x4(&self) -> bool {
        *self == CMD_A::_0X4
    }
    #[doc = "Checks if the value of the field is `_0X5`"]
    #[inline(always)]
    pub fn is_0x5(&self) -> bool {
        *self == CMD_A::_0X5
    }
    #[doc = "Checks if the value of the field is `_0X6`"]
    #[inline(always)]
    pub fn is_0x6(&self) -> bool {
        *self == CMD_A::_0X6
    }
    #[doc = "Checks if the value of the field is `_0X_B`"]
    #[inline(always)]
    pub fn is_0x_b(&self) -> bool {
        *self == CMD_A::_0X_B
    }
}
#[doc = "Field `CMD` writer - Software Command Setting"]
pub type CMD_W<'a, const O: u8> = crate::FieldWriter<'a, u8, FCR_SPEC, u8, CMD_A, 4, O>;
impl<'a, const O: u8> CMD_W<'a, O> {
    #[doc = "Program"]
    #[inline(always)]
    pub fn _0x1(self) -> &'a mut W {
        self.variant(CMD_A::_0X1)
    }
    #[doc = "Blank check (code flash)"]
    #[inline(always)]
    pub fn _0x3(self) -> &'a mut W {
        self.variant(CMD_A::_0X3)
    }
    #[doc = "Block erase"]
    #[inline(always)]
    pub fn _0x4(self) -> &'a mut W {
        self.variant(CMD_A::_0X4)
    }
    #[doc = "Consecutive read"]
    #[inline(always)]
    pub fn _0x5(self) -> &'a mut W {
        self.variant(CMD_A::_0X5)
    }
    #[doc = "Chip erase"]
    #[inline(always)]
    pub fn _0x6(self) -> &'a mut W {
        self.variant(CMD_A::_0X6)
    }
    #[doc = "Blank check (data flash)"]
    #[inline(always)]
    pub fn _0x_b(self) -> &'a mut W {
        self.variant(CMD_A::_0X_B)
    }
}
#[doc = "Field `DRC` reader - Data Read Completion"]
pub type DRC_R = crate::BitReader<DRC_A>;
#[doc = "Data Read Completion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRC_A {
    #[doc = "0: Data is not read or next data is requested"]
    _0 = 0,
    #[doc = "1: Data reading is complete."]
    _1 = 1,
}
impl From<DRC_A> for bool {
    #[inline(always)]
    fn from(variant: DRC_A) -> Self {
        variant as u8 != 0
    }
}
impl DRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRC_A {
        match self.bits {
            false => DRC_A::_0,
            true => DRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DRC_A::_1
    }
}
#[doc = "Field `DRC` writer - Data Read Completion"]
pub type DRC_W<'a, const O: u8> = crate::BitWriter<'a, u8, FCR_SPEC, DRC_A, O>;
impl<'a, const O: u8> DRC_W<'a, O> {
    #[doc = "Data is not read or next data is requested"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DRC_A::_0)
    }
    #[doc = "Data reading is complete."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DRC_A::_1)
    }
}
#[doc = "Field `STOP` reader - Forced Processing Stop"]
pub type STOP_R = crate::BitReader<bool>;
#[doc = "Field `STOP` writer - Forced Processing Stop"]
pub type STOP_W<'a, const O: u8> = crate::BitWriter<'a, u8, FCR_SPEC, bool, O>;
#[doc = "Field `OPST` reader - Processing Start"]
pub type OPST_R = crate::BitReader<OPST_A>;
#[doc = "Processing Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPST_A {
    #[doc = "0: Processing stops"]
    _0 = 0,
    #[doc = "1: Processing starts."]
    _1 = 1,
}
impl From<OPST_A> for bool {
    #[inline(always)]
    fn from(variant: OPST_A) -> Self {
        variant as u8 != 0
    }
}
impl OPST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OPST_A {
        match self.bits {
            false => OPST_A::_0,
            true => OPST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OPST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OPST_A::_1
    }
}
#[doc = "Field `OPST` writer - Processing Start"]
pub type OPST_W<'a, const O: u8> = crate::BitWriter<'a, u8, FCR_SPEC, OPST_A, O>;
impl<'a, const O: u8> OPST_W<'a, O> {
    #[doc = "Processing stops"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OPST_A::_0)
    }
    #[doc = "Processing starts."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OPST_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - Software Command Setting"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new(self.bits & 0x0f)
    }
    #[doc = "Bit 4 - Data Read Completion"]
    #[inline(always)]
    pub fn drc(&self) -> DRC_R {
        DRC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Forced Processing Stop"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Processing Start"]
    #[inline(always)]
    pub fn opst(&self) -> OPST_R {
        OPST_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Software Command Setting"]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CMD_W<0> {
        CMD_W::new(self)
    }
    #[doc = "Bit 4 - Data Read Completion"]
    #[inline(always)]
    #[must_use]
    pub fn drc(&mut self) -> DRC_W<4> {
        DRC_W::new(self)
    }
    #[doc = "Bit 6 - Forced Processing Stop"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<6> {
        STOP_W::new(self)
    }
    #[doc = "Bit 7 - Processing Start"]
    #[inline(always)]
    #[must_use]
    pub fn opst(&mut self) -> OPST_W<7> {
        OPST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcr](index.html) module"]
pub struct FCR_SPEC;
impl crate::RegisterSpec for FCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fcr::R](R) reader structure"]
impl crate::Readable for FCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcr::W](W) writer structure"]
impl crate::Writable for FCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCR to value 0"]
impl crate::Resettable for FCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

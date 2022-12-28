#[doc = "Register `SCKDIVCR` reader"]
pub struct R(crate::R<SCKDIVCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCKDIVCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCKDIVCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCKDIVCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCKDIVCR` writer"]
pub struct W(crate::W<SCKDIVCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCKDIVCR_SPEC>;
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
impl From<crate::W<SCKDIVCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCKDIVCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCKD` reader - Peripheral Module Clock D (PCLKD) Select"]
pub type PCKD_R = crate::FieldReader<u8, PCKD_A>;
#[doc = "Peripheral Module Clock D (PCLKD) Select\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCKD_A {
    #[doc = "0: x 1/1"]
    _000 = 0,
    #[doc = "1: x 1/2"]
    _001 = 1,
    #[doc = "2: x 1/4"]
    _010 = 2,
    #[doc = "3: x 1/8"]
    _011 = 3,
    #[doc = "4: x 1/16"]
    _100 = 4,
    #[doc = "5: x 1/32"]
    _101 = 5,
    #[doc = "6: x 1/64"]
    _110 = 6,
}
impl From<PCKD_A> for u8 {
    #[inline(always)]
    fn from(variant: PCKD_A) -> Self {
        variant as _
    }
}
impl PCKD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCKD_A {
        match self.bits {
            0 => PCKD_A::_000,
            1 => PCKD_A::_001,
            2 => PCKD_A::_010,
            3 => PCKD_A::_011,
            4 => PCKD_A::_100,
            5 => PCKD_A::_101,
            6 => PCKD_A::_110,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == PCKD_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == PCKD_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == PCKD_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == PCKD_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == PCKD_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == PCKD_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == PCKD_A::_110
    }
}
#[doc = "Field `PCKD` writer - Peripheral Module Clock D (PCLKD) Select"]
pub type PCKD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCKDIVCR_SPEC, u8, PCKD_A, 3, O>;
impl<'a, const O: u8> PCKD_W<'a, O> {
    #[doc = "x 1/1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(PCKD_A::_000)
    }
    #[doc = "x 1/2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(PCKD_A::_001)
    }
    #[doc = "x 1/4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(PCKD_A::_010)
    }
    #[doc = "x 1/8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(PCKD_A::_011)
    }
    #[doc = "x 1/16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(PCKD_A::_100)
    }
    #[doc = "x 1/32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(PCKD_A::_101)
    }
    #[doc = "x 1/64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(PCKD_A::_110)
    }
}
#[doc = "Field `PCKB` reader - Peripheral Module Clock B (PCLKB) Select"]
pub type PCKB_R = crate::FieldReader<u8, PCKB_A>;
#[doc = "Peripheral Module Clock B (PCLKB) Select\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCKB_A {
    #[doc = "0: x 1/1"]
    _000 = 0,
    #[doc = "1: x 1/2"]
    _001 = 1,
    #[doc = "2: x 1/4"]
    _010 = 2,
    #[doc = "3: x 1/8"]
    _011 = 3,
    #[doc = "4: x 1/16"]
    _100 = 4,
    #[doc = "5: x 1/32"]
    _101 = 5,
    #[doc = "6: x 1/64"]
    _110 = 6,
}
impl From<PCKB_A> for u8 {
    #[inline(always)]
    fn from(variant: PCKB_A) -> Self {
        variant as _
    }
}
impl PCKB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCKB_A {
        match self.bits {
            0 => PCKB_A::_000,
            1 => PCKB_A::_001,
            2 => PCKB_A::_010,
            3 => PCKB_A::_011,
            4 => PCKB_A::_100,
            5 => PCKB_A::_101,
            6 => PCKB_A::_110,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == PCKB_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == PCKB_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == PCKB_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == PCKB_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == PCKB_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == PCKB_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == PCKB_A::_110
    }
}
#[doc = "Field `PCKB` writer - Peripheral Module Clock B (PCLKB) Select"]
pub type PCKB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCKDIVCR_SPEC, u8, PCKB_A, 3, O>;
impl<'a, const O: u8> PCKB_W<'a, O> {
    #[doc = "x 1/1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(PCKB_A::_000)
    }
    #[doc = "x 1/2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(PCKB_A::_001)
    }
    #[doc = "x 1/4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(PCKB_A::_010)
    }
    #[doc = "x 1/8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(PCKB_A::_011)
    }
    #[doc = "x 1/16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(PCKB_A::_100)
    }
    #[doc = "x 1/32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(PCKB_A::_101)
    }
    #[doc = "x 1/64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(PCKB_A::_110)
    }
}
#[doc = "Field `ICK` reader - System Clock (ICLK) Select"]
pub type ICK_R = crate::FieldReader<u8, ICK_A>;
#[doc = "System Clock (ICLK) Select\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICK_A {
    #[doc = "0: x 1/1"]
    _000 = 0,
    #[doc = "1: x 1/2"]
    _001 = 1,
    #[doc = "2: x 1/4"]
    _010 = 2,
    #[doc = "3: x 1/8"]
    _011 = 3,
    #[doc = "4: x 1/16"]
    _100 = 4,
    #[doc = "5: x 1/32"]
    _101 = 5,
    #[doc = "6: x 1/64"]
    _110 = 6,
}
impl From<ICK_A> for u8 {
    #[inline(always)]
    fn from(variant: ICK_A) -> Self {
        variant as _
    }
}
impl ICK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICK_A {
        match self.bits {
            0 => ICK_A::_000,
            1 => ICK_A::_001,
            2 => ICK_A::_010,
            3 => ICK_A::_011,
            4 => ICK_A::_100,
            5 => ICK_A::_101,
            6 => ICK_A::_110,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == ICK_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == ICK_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == ICK_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == ICK_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == ICK_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == ICK_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == ICK_A::_110
    }
}
#[doc = "Field `ICK` writer - System Clock (ICLK) Select"]
pub type ICK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCKDIVCR_SPEC, u8, ICK_A, 3, O>;
impl<'a, const O: u8> ICK_W<'a, O> {
    #[doc = "x 1/1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(ICK_A::_000)
    }
    #[doc = "x 1/2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(ICK_A::_001)
    }
    #[doc = "x 1/4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(ICK_A::_010)
    }
    #[doc = "x 1/8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(ICK_A::_011)
    }
    #[doc = "x 1/16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(ICK_A::_100)
    }
    #[doc = "x 1/32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(ICK_A::_101)
    }
    #[doc = "x 1/64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(ICK_A::_110)
    }
}
impl R {
    #[doc = "Bits 0:2 - Peripheral Module Clock D (PCLKD) Select"]
    #[inline(always)]
    pub fn pckd(&self) -> PCKD_R {
        PCKD_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - Peripheral Module Clock B (PCLKB) Select"]
    #[inline(always)]
    pub fn pckb(&self) -> PCKB_R {
        PCKB_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 24:26 - System Clock (ICLK) Select"]
    #[inline(always)]
    pub fn ick(&self) -> ICK_R {
        ICK_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Peripheral Module Clock D (PCLKD) Select"]
    #[inline(always)]
    #[must_use]
    pub fn pckd(&mut self) -> PCKD_W<0> {
        PCKD_W::new(self)
    }
    #[doc = "Bits 8:10 - Peripheral Module Clock B (PCLKB) Select"]
    #[inline(always)]
    #[must_use]
    pub fn pckb(&mut self) -> PCKB_W<8> {
        PCKB_W::new(self)
    }
    #[doc = "Bits 24:26 - System Clock (ICLK) Select"]
    #[inline(always)]
    #[must_use]
    pub fn ick(&mut self) -> ICK_W<24> {
        ICK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Clock Division Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sckdivcr](index.html) module"]
pub struct SCKDIVCR_SPEC;
impl crate::RegisterSpec for SCKDIVCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sckdivcr::R](R) reader structure"]
impl crate::Readable for SCKDIVCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sckdivcr::W](W) writer structure"]
impl crate::Writable for SCKDIVCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCKDIVCR to value 0x0400_0404"]
impl crate::Resettable for SCKDIVCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0400_0404;
}

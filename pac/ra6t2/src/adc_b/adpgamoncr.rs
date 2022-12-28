#[doc = "Register `ADPGAMONCR` reader"]
pub struct R(crate::R<ADPGAMONCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPGAMONCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPGAMONCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPGAMONCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADPGAMONCR` writer"]
pub struct W(crate::W<ADPGAMONCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPGAMONCR_SPEC>;
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
impl From<crate::W<ADPGAMONCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPGAMONCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PGAMON` reader - PGA Monitor Signal Selection"]
pub type PGAMON_R = crate::FieldReader<u8, PGAMON_A>;
#[doc = "PGA Monitor Signal Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PGAMON_A {
    #[doc = "0: Not select monitor signal (Hi-Z)"]
    _0X0 = 0,
    #[doc = "1: PGA output"]
    _0X1 = 1,
}
impl From<PGAMON_A> for u8 {
    #[inline(always)]
    fn from(variant: PGAMON_A) -> Self {
        variant as _
    }
}
impl PGAMON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PGAMON_A> {
        match self.bits {
            0 => Some(PGAMON_A::_0X0),
            1 => Some(PGAMON_A::_0X1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X0`"]
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == PGAMON_A::_0X0
    }
    #[doc = "Checks if the value of the field is `_0X1`"]
    #[inline(always)]
    pub fn is_0x1(&self) -> bool {
        *self == PGAMON_A::_0X1
    }
}
#[doc = "Field `PGAMON` writer - PGA Monitor Signal Selection"]
pub type PGAMON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADPGAMONCR_SPEC, u8, PGAMON_A, 3, O>;
impl<'a, const O: u8> PGAMON_W<'a, O> {
    #[doc = "Not select monitor signal (Hi-Z)"]
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut W {
        self.variant(PGAMON_A::_0X0)
    }
    #[doc = "PGA output"]
    #[inline(always)]
    pub fn _0x1(self) -> &'a mut W {
        self.variant(PGAMON_A::_0X1)
    }
}
#[doc = "Field `MONSEL0` reader - PGA Unit 0 Monitor Output Enable"]
pub type MONSEL0_R = crate::BitReader<MONSEL0_A>;
#[doc = "PGA Unit 0 Monitor Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MONSEL0_A {
    #[doc = "0: Disable monitor output"]
    _0 = 0,
    #[doc = "1: Enable monitor output"]
    _1 = 1,
}
impl From<MONSEL0_A> for bool {
    #[inline(always)]
    fn from(variant: MONSEL0_A) -> Self {
        variant as u8 != 0
    }
}
impl MONSEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MONSEL0_A {
        match self.bits {
            false => MONSEL0_A::_0,
            true => MONSEL0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MONSEL0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MONSEL0_A::_1
    }
}
#[doc = "Field `MONSEL0` writer - PGA Unit 0 Monitor Output Enable"]
pub type MONSEL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADPGAMONCR_SPEC, MONSEL0_A, O>;
impl<'a, const O: u8> MONSEL0_W<'a, O> {
    #[doc = "Disable monitor output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MONSEL0_A::_0)
    }
    #[doc = "Enable monitor output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MONSEL0_A::_1)
    }
}
#[doc = "Field `MONSEL1` reader - PGA Unit 1 Monitor Output Enable"]
pub type MONSEL1_R = crate::BitReader<MONSEL1_A>;
#[doc = "PGA Unit 1 Monitor Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MONSEL1_A {
    #[doc = "0: Disable monitor output"]
    _0 = 0,
    #[doc = "1: Enable monitor output"]
    _1 = 1,
}
impl From<MONSEL1_A> for bool {
    #[inline(always)]
    fn from(variant: MONSEL1_A) -> Self {
        variant as u8 != 0
    }
}
impl MONSEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MONSEL1_A {
        match self.bits {
            false => MONSEL1_A::_0,
            true => MONSEL1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MONSEL1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MONSEL1_A::_1
    }
}
#[doc = "Field `MONSEL1` writer - PGA Unit 1 Monitor Output Enable"]
pub type MONSEL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADPGAMONCR_SPEC, MONSEL1_A, O>;
impl<'a, const O: u8> MONSEL1_W<'a, O> {
    #[doc = "Disable monitor output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MONSEL1_A::_0)
    }
    #[doc = "Enable monitor output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MONSEL1_A::_1)
    }
}
#[doc = "Field `MONSEL2` reader - PGA Unit 2 Monitor Output Enable"]
pub type MONSEL2_R = crate::BitReader<MONSEL2_A>;
#[doc = "PGA Unit 2 Monitor Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MONSEL2_A {
    #[doc = "0: Disable monitor output"]
    _0 = 0,
    #[doc = "1: Enable monitor output"]
    _1 = 1,
}
impl From<MONSEL2_A> for bool {
    #[inline(always)]
    fn from(variant: MONSEL2_A) -> Self {
        variant as u8 != 0
    }
}
impl MONSEL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MONSEL2_A {
        match self.bits {
            false => MONSEL2_A::_0,
            true => MONSEL2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MONSEL2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MONSEL2_A::_1
    }
}
#[doc = "Field `MONSEL2` writer - PGA Unit 2 Monitor Output Enable"]
pub type MONSEL2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADPGAMONCR_SPEC, MONSEL2_A, O>;
impl<'a, const O: u8> MONSEL2_W<'a, O> {
    #[doc = "Disable monitor output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MONSEL2_A::_0)
    }
    #[doc = "Enable monitor output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MONSEL2_A::_1)
    }
}
#[doc = "Field `MONSEL3` reader - PGA Unit 3 Monitor Output Enable"]
pub type MONSEL3_R = crate::BitReader<MONSEL3_A>;
#[doc = "PGA Unit 3 Monitor Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MONSEL3_A {
    #[doc = "0: Disable monitor output"]
    _0 = 0,
    #[doc = "1: Enable monitor output"]
    _1 = 1,
}
impl From<MONSEL3_A> for bool {
    #[inline(always)]
    fn from(variant: MONSEL3_A) -> Self {
        variant as u8 != 0
    }
}
impl MONSEL3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MONSEL3_A {
        match self.bits {
            false => MONSEL3_A::_0,
            true => MONSEL3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MONSEL3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MONSEL3_A::_1
    }
}
#[doc = "Field `MONSEL3` writer - PGA Unit 3 Monitor Output Enable"]
pub type MONSEL3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADPGAMONCR_SPEC, MONSEL3_A, O>;
impl<'a, const O: u8> MONSEL3_W<'a, O> {
    #[doc = "Disable monitor output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MONSEL3_A::_0)
    }
    #[doc = "Enable monitor output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MONSEL3_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - PGA Monitor Signal Selection"]
    #[inline(always)]
    pub fn pgamon(&self) -> PGAMON_R {
        PGAMON_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 16 - PGA Unit 0 Monitor Output Enable"]
    #[inline(always)]
    pub fn monsel0(&self) -> MONSEL0_R {
        MONSEL0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PGA Unit 1 Monitor Output Enable"]
    #[inline(always)]
    pub fn monsel1(&self) -> MONSEL1_R {
        MONSEL1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PGA Unit 2 Monitor Output Enable"]
    #[inline(always)]
    pub fn monsel2(&self) -> MONSEL2_R {
        MONSEL2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PGA Unit 3 Monitor Output Enable"]
    #[inline(always)]
    pub fn monsel3(&self) -> MONSEL3_R {
        MONSEL3_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - PGA Monitor Signal Selection"]
    #[inline(always)]
    #[must_use]
    pub fn pgamon(&mut self) -> PGAMON_W<0> {
        PGAMON_W::new(self)
    }
    #[doc = "Bit 16 - PGA Unit 0 Monitor Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn monsel0(&mut self) -> MONSEL0_W<16> {
        MONSEL0_W::new(self)
    }
    #[doc = "Bit 17 - PGA Unit 1 Monitor Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn monsel1(&mut self) -> MONSEL1_W<17> {
        MONSEL1_W::new(self)
    }
    #[doc = "Bit 18 - PGA Unit 2 Monitor Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn monsel2(&mut self) -> MONSEL2_W<18> {
        MONSEL2_W::new(self)
    }
    #[doc = "Bit 19 - PGA Unit 3 Monitor Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn monsel3(&mut self) -> MONSEL3_W<19> {
        MONSEL3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Programable Gain Amp Monitor Output Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpgamoncr](index.html) module"]
pub struct ADPGAMONCR_SPEC;
impl crate::RegisterSpec for ADPGAMONCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpgamoncr::R](R) reader structure"]
impl crate::Readable for ADPGAMONCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpgamoncr::W](W) writer structure"]
impl crate::Writable for ADPGAMONCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADPGAMONCR to value 0"]
impl crate::Resettable for ADPGAMONCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

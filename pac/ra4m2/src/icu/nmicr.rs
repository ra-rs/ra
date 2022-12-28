#[doc = "Register `NMICR` reader"]
pub struct R(crate::R<NMICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NMICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NMICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NMICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NMICR` writer"]
pub struct W(crate::W<NMICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NMICR_SPEC>;
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
impl From<crate::W<NMICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NMICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NMIMD` reader - NMI Detection Set"]
pub type NMIMD_R = crate::BitReader<NMIMD_A>;
#[doc = "NMI Detection Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NMIMD_A {
    #[doc = "0: Falling edge"]
    _0 = 0,
    #[doc = "1: Rising edge"]
    _1 = 1,
}
impl From<NMIMD_A> for bool {
    #[inline(always)]
    fn from(variant: NMIMD_A) -> Self {
        variant as u8 != 0
    }
}
impl NMIMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NMIMD_A {
        match self.bits {
            false => NMIMD_A::_0,
            true => NMIMD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NMIMD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NMIMD_A::_1
    }
}
#[doc = "Field `NMIMD` writer - NMI Detection Set"]
pub type NMIMD_W<'a, const O: u8> = crate::BitWriter<'a, u8, NMICR_SPEC, NMIMD_A, O>;
impl<'a, const O: u8> NMIMD_W<'a, O> {
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NMIMD_A::_0)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NMIMD_A::_1)
    }
}
#[doc = "Field `NFCLKSEL` reader - NMI Digital Filter Sampling Clock Select"]
pub type NFCLKSEL_R = crate::FieldReader<u8, NFCLKSEL_A>;
#[doc = "NMI Digital Filter Sampling Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NFCLKSEL_A {
    #[doc = "0: PCLKB"]
    _00 = 0,
    #[doc = "1: PCLKB/8"]
    _01 = 1,
    #[doc = "2: PCLKB/32"]
    _10 = 2,
    #[doc = "3: PCLKB/64"]
    _11 = 3,
}
impl From<NFCLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: NFCLKSEL_A) -> Self {
        variant as _
    }
}
impl NFCLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NFCLKSEL_A {
        match self.bits {
            0 => NFCLKSEL_A::_00,
            1 => NFCLKSEL_A::_01,
            2 => NFCLKSEL_A::_10,
            3 => NFCLKSEL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == NFCLKSEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == NFCLKSEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == NFCLKSEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == NFCLKSEL_A::_11
    }
}
#[doc = "Field `NFCLKSEL` writer - NMI Digital Filter Sampling Clock Select"]
pub type NFCLKSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, NMICR_SPEC, u8, NFCLKSEL_A, 2, O>;
impl<'a, const O: u8> NFCLKSEL_W<'a, O> {
    #[doc = "PCLKB"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(NFCLKSEL_A::_00)
    }
    #[doc = "PCLKB/8"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(NFCLKSEL_A::_01)
    }
    #[doc = "PCLKB/32"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(NFCLKSEL_A::_10)
    }
    #[doc = "PCLKB/64"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(NFCLKSEL_A::_11)
    }
}
#[doc = "Field `NFLTEN` reader - NMI Digital Filter Enable"]
pub type NFLTEN_R = crate::BitReader<NFLTEN_A>;
#[doc = "NMI Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NFLTEN_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<NFLTEN_A> for bool {
    #[inline(always)]
    fn from(variant: NFLTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl NFLTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NFLTEN_A {
        match self.bits {
            false => NFLTEN_A::_0,
            true => NFLTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NFLTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NFLTEN_A::_1
    }
}
#[doc = "Field `NFLTEN` writer - NMI Digital Filter Enable"]
pub type NFLTEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, NMICR_SPEC, NFLTEN_A, O>;
impl<'a, const O: u8> NFLTEN_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NFLTEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NFLTEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - NMI Detection Set"]
    #[inline(always)]
    pub fn nmimd(&self) -> NMIMD_R {
        NMIMD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - NMI Digital Filter Sampling Clock Select"]
    #[inline(always)]
    pub fn nfclksel(&self) -> NFCLKSEL_R {
        NFCLKSEL_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 7 - NMI Digital Filter Enable"]
    #[inline(always)]
    pub fn nflten(&self) -> NFLTEN_R {
        NFLTEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NMI Detection Set"]
    #[inline(always)]
    #[must_use]
    pub fn nmimd(&mut self) -> NMIMD_W<0> {
        NMIMD_W::new(self)
    }
    #[doc = "Bits 4:5 - NMI Digital Filter Sampling Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn nfclksel(&mut self) -> NFCLKSEL_W<4> {
        NFCLKSEL_W::new(self)
    }
    #[doc = "Bit 7 - NMI Digital Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nflten(&mut self) -> NFLTEN_W<7> {
        NFLTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "NMI Pin Interrupt Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nmicr](index.html) module"]
pub struct NMICR_SPEC;
impl crate::RegisterSpec for NMICR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [nmicr::R](R) reader structure"]
impl crate::Readable for NMICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nmicr::W](W) writer structure"]
impl crate::Writable for NMICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NMICR to value 0"]
impl crate::Resettable for NMICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `FAEINT` reader"]
pub struct R(crate::R<FAEINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FAEINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FAEINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FAEINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FAEINT` writer"]
pub struct W(crate::W<FAEINT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FAEINT_SPEC>;
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
impl From<crate::W<FAEINT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FAEINT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DFAEIE` reader - Data Flash Memory Access Violation Interrupt Enable"]
pub type DFAEIE_R = crate::BitReader<DFAEIE_A>;
#[doc = "Data Flash Memory Access Violation Interrupt Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFAEIE_A {
    #[doc = "0: Generation of an FIFERR interrupt request is disabled when FASTAT.DFAE is set to 1"]
    _0 = 0,
    #[doc = "1: Generation of an FIFERR interrupt request is enabled when FASTAT.DFAE is set to 1."]
    _1 = 1,
}
impl From<DFAEIE_A> for bool {
    #[inline(always)]
    fn from(variant: DFAEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl DFAEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFAEIE_A {
        match self.bits {
            false => DFAEIE_A::_0,
            true => DFAEIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFAEIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFAEIE_A::_1
    }
}
#[doc = "Field `DFAEIE` writer - Data Flash Memory Access Violation Interrupt Enable"]
pub type DFAEIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, FAEINT_SPEC, DFAEIE_A, O>;
impl<'a, const O: u8> DFAEIE_W<'a, O> {
    #[doc = "Generation of an FIFERR interrupt request is disabled when FASTAT.DFAE is set to 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFAEIE_A::_0)
    }
    #[doc = "Generation of an FIFERR interrupt request is enabled when FASTAT.DFAE is set to 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFAEIE_A::_1)
    }
}
#[doc = "Field `CMDLKIE` reader - Command Lock Interrupt Enable"]
pub type CMDLKIE_R = crate::BitReader<CMDLKIE_A>;
#[doc = "Command Lock Interrupt Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDLKIE_A {
    #[doc = "0: Generation of an FIFERR interrupt request is disabled when FASTAT.CMDLK is set to 1"]
    _0 = 0,
    #[doc = "1: Generation of an FIFERR interrupt request is enabled when FASTAT.CMDLK is set to 1."]
    _1 = 1,
}
impl From<CMDLKIE_A> for bool {
    #[inline(always)]
    fn from(variant: CMDLKIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CMDLKIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDLKIE_A {
        match self.bits {
            false => CMDLKIE_A::_0,
            true => CMDLKIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMDLKIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMDLKIE_A::_1
    }
}
#[doc = "Field `CMDLKIE` writer - Command Lock Interrupt Enable"]
pub type CMDLKIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, FAEINT_SPEC, CMDLKIE_A, O>;
impl<'a, const O: u8> CMDLKIE_W<'a, O> {
    #[doc = "Generation of an FIFERR interrupt request is disabled when FASTAT.CMDLK is set to 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMDLKIE_A::_0)
    }
    #[doc = "Generation of an FIFERR interrupt request is enabled when FASTAT.CMDLK is set to 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMDLKIE_A::_1)
    }
}
#[doc = "Field `CFAEIE` reader - Code Flash Memory Access Violation Interrupt Enable"]
pub type CFAEIE_R = crate::BitReader<CFAEIE_A>;
#[doc = "Code Flash Memory Access Violation Interrupt Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFAEIE_A {
    #[doc = "0: Generation of an FIFERR interrupt request is disabled when FASTAT.CFAE is set to 1"]
    _0 = 0,
    #[doc = "1: Generation of an FIFERR interrupt request is enabled when FASTAT.CFAE is set to 1."]
    _1 = 1,
}
impl From<CFAEIE_A> for bool {
    #[inline(always)]
    fn from(variant: CFAEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CFAEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFAEIE_A {
        match self.bits {
            false => CFAEIE_A::_0,
            true => CFAEIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CFAEIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CFAEIE_A::_1
    }
}
#[doc = "Field `CFAEIE` writer - Code Flash Memory Access Violation Interrupt Enable"]
pub type CFAEIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, FAEINT_SPEC, CFAEIE_A, O>;
impl<'a, const O: u8> CFAEIE_W<'a, O> {
    #[doc = "Generation of an FIFERR interrupt request is disabled when FASTAT.CFAE is set to 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFAEIE_A::_0)
    }
    #[doc = "Generation of an FIFERR interrupt request is enabled when FASTAT.CFAE is set to 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFAEIE_A::_1)
    }
}
impl R {
    #[doc = "Bit 3 - Data Flash Memory Access Violation Interrupt Enable"]
    #[inline(always)]
    pub fn dfaeie(&self) -> DFAEIE_R {
        DFAEIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Command Lock Interrupt Enable"]
    #[inline(always)]
    pub fn cmdlkie(&self) -> CMDLKIE_R {
        CMDLKIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Code Flash Memory Access Violation Interrupt Enable"]
    #[inline(always)]
    pub fn cfaeie(&self) -> CFAEIE_R {
        CFAEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Data Flash Memory Access Violation Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfaeie(&mut self) -> DFAEIE_W<3> {
        DFAEIE_W::new(self)
    }
    #[doc = "Bit 4 - Command Lock Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdlkie(&mut self) -> CMDLKIE_W<4> {
        CMDLKIE_W::new(self)
    }
    #[doc = "Bit 7 - Code Flash Memory Access Violation Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfaeie(&mut self) -> CFAEIE_W<7> {
        CFAEIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Access Error Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [faeint](index.html) module"]
pub struct FAEINT_SPEC;
impl crate::RegisterSpec for FAEINT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [faeint::R](R) reader structure"]
impl crate::Readable for FAEINT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [faeint::W](W) writer structure"]
impl crate::Writable for FAEINT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FAEINT to value 0x98"]
impl crate::Resettable for FAEINT_SPEC {
    const RESET_VALUE: Self::Ux = 0x98;
}
